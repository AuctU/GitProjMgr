use std::{collections::HashMap, ops, path, sync, thread, time};

use anyhow::{bail, Context, Result};
use futures::executor::block_on;
use tokio::sync::{Mutex, Semaphore};

use crate::{
    bookmarks, database, deltas, events, files, gb_repository,
    project_repository::{self, activity, branch, conflicts, diff},
    projects, pty, search, sessions, storage, users, virtual_branches, watcher,
};

#[derive(Clone)]
pub struct App {
    local_data_dir: std::path::PathBuf,

    projects_storage: projects::Storage,
    users_storage: users::Storage,

    searcher: search::Searcher,
    events_sender: events::Sender,

    watchers: sync::Arc<Mutex<HashMap<String, watcher::Watcher>>>,

    sessions_database: sessions::Database,
    files_database: files::Database,
    deltas_database: deltas::Database,
    bookmarks_database: bookmarks::Database,

    vbranch_semaphores: sync::Arc<tokio::sync::Mutex<HashMap<String, Semaphore>>>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Message(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl App {
    pub fn new<P: AsRef<std::path::Path>>(
        local_data_dir: P,
        event_sender: events::Sender,
    ) -> Result<Self> {
        let local_data_dir = local_data_dir.as_ref();
        let storage = storage::Storage::from_path(local_data_dir);
        let deltas_searcher =
            search::Searcher::at(local_data_dir).context("failed to open deltas searcher")?;
        let database = database::Database::open(local_data_dir.join("database.sqlite3"))?;
        Ok(Self {
            events_sender: event_sender,
            local_data_dir: local_data_dir.to_path_buf(),
            projects_storage: projects::Storage::new(storage.clone()),
            users_storage: users::Storage::new(storage),
            searcher: deltas_searcher,
            watchers: sync::Arc::new(Mutex::new(HashMap::new())),
            sessions_database: sessions::Database::new(database.clone()),
            deltas_database: deltas::Database::new(database.clone()),
            files_database: files::Database::new(database.clone()),
            bookmarks_database: bookmarks::Database::new(database),
            vbranch_semaphores: sync::Arc::new(tokio::sync::Mutex::new(HashMap::new())),
        })
    }

    pub fn start_pty_server(&self) -> Result<()> {
        let self_ = self.clone();
        tauri::async_runtime::spawn(async move {
            let port = if cfg!(debug_assertions) { 7702 } else { 7703 };
            if let Err(e) = pty::start_server(port, self_).await {
                log::error!("failed to start pty server: {:#}", e);
            }
        });
        Ok(())
    }

    pub fn init_project(&self, project: &projects::Project) -> Result<()> {
        block_on(async move {
            self.start_watcher(project)
                .await
                .with_context(|| {
                    format!("failed to start watcher for project {}", project.id.clone())
                })
                .expect("failed to start watcher");
        });

        Ok(())
    }

    pub fn init(&self) -> Result<()> {
        for project in self
            .projects_storage
            .list_projects()
            .with_context(|| "failed to list projects")?
        {
            if let Err(e) = self.init_project(&project) {
                log::error!("failed to init project {}: {:#}", project.id, e);
            }
        }
        Ok(())
    }

    async fn start_watcher(&self, project: &projects::Project) -> Result<()> {
        let watcher = watcher::Watcher::new(
            &self.local_data_dir,
            project,
            &self.projects_storage,
            &self.users_storage,
            &self.searcher,
            &self.events_sender,
            &self.sessions_database,
            &self.deltas_database,
            &self.files_database,
            &self.bookmarks_database,
        );

        let c_watcher = watcher.clone();
        let project_id = project.id.clone();
        thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .thread_name(format!("watcher-{}", project_id))
                .enable_time()
                .build()
                .unwrap();
            rt.block_on(async move {
                if let Err(e) = c_watcher.run().await {
                    log::error!("watcher error: {:#}", e);
                }
                log::info!("watcher stopped");
            });
        });

        self.watchers
            .lock()
            .await
            .insert(project.id.clone(), watcher.clone());

        Ok(())
    }

    async fn send_event(&self, project_id: &str, event: watcher::Event) -> Result<()> {
        let watchers = self.watchers.lock().await;
        if let Some(watcher) = watchers.get(project_id) {
            watcher.post(event).await.context("failed to post event")
        } else {
            Err(anyhow::anyhow!(
                "watcher for project {} not found",
                project_id
            ))
        }
    }

    async fn stop_watcher(&self, project_id: &str) -> Result<()> {
        if let Some((_, watcher)) = self.watchers.lock().await.remove_entry(project_id) {
            watcher.stop()?;
        };
        Ok(())
    }

    fn gb_repository(&self, project_id: &str) -> Result<gb_repository::Repository> {
        gb_repository::Repository::open(
            self.local_data_dir.clone(),
            project_id.to_string(),
            self.projects_storage.clone(),
            self.users_storage.clone(),
        )
        .context("failed to open repository")
    }

    fn gb_project(&self, project_id: &str) -> Result<projects::Project> {
        self.projects_storage
            .get_project(project_id)
            .context("failed to get project")?
            .ok_or_else(|| anyhow::anyhow!("project {} not found", project_id))
    }

    pub fn get_user(&self) -> Result<Option<users::User>> {
        self.users_storage.get()
    }

    pub fn set_user(&self, user: &users::User) -> Result<()> {
        self.users_storage.set(user)
    }

    pub fn delete_user(&self) -> Result<()> {
        self.users_storage.delete()
    }

    pub fn add_project(&self, path: &str) -> Result<projects::Project, Error> {
        let all_projects = self
            .projects_storage
            .list_projects()
            .map_err(Error::Other)?;

        if all_projects.iter().any(|project| project.path == path) {
            return Err(Error::Message("Project already exists".to_string()));
        }

        let project = projects::Project::from_path(path.to_string())
            .map_err(|err| Error::Message(err.to_string()))?;

        self.projects_storage
            .add_project(&project)
            .context("failed to add project")
            .map_err(Error::Other)?;

        self.init_project(&project)
            .context("failed to init project")
            .map_err(Error::Other)?;

        Ok(project)
    }

    pub fn update_project(&self, project: &projects::UpdateRequest) -> Result<projects::Project> {
        let updated = self.projects_storage.update_project(project)?;

        block_on(async move {
            if let Err(err) = self
                .send_event(
                    &project.id,
                    watcher::Event::FetchGitbutlerData(time::SystemTime::now()),
                )
                .await
            {
                log::error!("{}: failed to fetch project: {:#}", &project.id, err);
            }
        });

        Ok(updated)
    }

    pub fn get_project(&self, id: &str) -> Result<Option<projects::Project>> {
        self.projects_storage.get_project(id)
    }

    pub fn list_projects(&self) -> Result<Vec<projects::Project>> {
        self.projects_storage.list_projects()
    }

    pub fn delete_project(&self, id: &str) -> Result<()> {
        match self.projects_storage.get_project(id)? {
            Some(project) => {
                let gb_repository = gb_repository::Repository::open(
                    self.local_data_dir.clone(),
                    id.to_string(),
                    self.projects_storage.clone(),
                    self.users_storage.clone(),
                )
                .context("failed to open repository")?;

                block_on({
                    let project_id = project.id.clone();
                    async move {
                        if let Err(e) = self.stop_watcher(&project_id).await {
                            log::error!("failed to stop watcher for project {}: {}", project_id, e);
                        }
                    }
                });

                if let Err(e) = gb_repository.purge() {
                    log::error!("failed to remove project dir {}: {}", project.id, e);
                }

                Ok(())
            }
            None => Ok(()),
        }
    }

    pub fn list_sessions(
        &self,
        project_id: &str,
        earliest_timestamp_ms: Option<u128>,
    ) -> Result<Vec<sessions::Session>> {
        self.sessions_database
            .list_by_project_id(project_id, earliest_timestamp_ms)
    }

    pub fn list_session_files(
        &self,
        project_id: &str,
        session_id: &str,
        paths: Option<Vec<&str>>,
    ) -> Result<HashMap<String, String>> {
        self.files_database
            .list_by_project_id_session_id(project_id, session_id, paths)
    }

    pub fn get_base_branch_data(
        &self,
        project_id: &str,
    ) -> Result<Option<virtual_branches::BaseBranch>> {
        let gb_repository = self.gb_repository(project_id)?;
        let project = self.gb_project(project_id)?;
        let project_repository = project_repository::Repository::open(&project)
            .context("failed to open project repository")?;
        virtual_branches::get_base_branch_data(&gb_repository, &project_repository)
    }

    pub async fn set_base_branch(
        &self,
        project_id: &str,
        target_branch: &str,
    ) -> Result<Option<virtual_branches::BaseBranch>> {
        let gb_repository = self.gb_repository(project_id)?;
        let project = self.gb_project(project_id)?;
        let project_repository = project_repository::Repository::open(&project)
            .context("failed to open project repository")?;

        let mut semaphores = self.vbranch_semaphores.lock().await;
        let semaphore = semaphores
            .entry(project_id.to_string())
            .or_insert_with(|| Semaphore::new(1));
        let _permit = semaphore.acquire().await?;

        let target = gb_repository.set_base_branch(&project_repository, target_branch)?;
        Ok(Some(target))
    }

    pub async fn update_base_branch(&self, project_id: &str) -> Result<()> {
        let gb_repository = self.gb_repository(project_id)?;
        let project = self.gb_project(project_id)?;
        let project_repository = project_repository::Repository::open(&project)
            .context("failed to open project repository")?;

        let mut semaphores = self.vbranch_semaphores.lock().await;
        let semaphore = semaphores
            .entry(project_id.to_string())
            .or_insert_with(|| Semaphore::new(1));
        let _permit = semaphore.acquire().await?;

        virtual_branches::update_base_branch(&gb_repository, &project_repository)?;
        Ok(())
    }

    pub async fn list_virtual_branches(
        &self,
        project_id: &str,
    ) -> Result<Vec<virtual_branches::VirtualBranch>> {
        let gb_repository = self.gb_repository(project_id)?;
        let project = self.gb_project(project_id)?;
        let project_repository = project_repository::Repository::open(&project)
            .context("failed to open project repository")?;

        let mut semaphores = self.vbranch_semaphores.lock().await;
        let semaphore = semaphores
            .entry(project_id.to_string())
            .or_insert_with(|| Semaphore::new(1));
        let _permit = semaphore.acquire().await?;

        virtual_branches::list_virtual_branches(&gb_repository, &project_repository)
    }

    pub async fn create_virtual_branch(
        &self,
        project_id: &str,
        create: &virtual_branches::branch::BranchCreateRequest,
    ) -> Result<()> {
        let gb_repository = self.gb_repository(project_id)?;

        let project = self.gb_project(project_id)?;
        let project_repository = project_repository::Repository::open(&project)
            .context("failed to open project repository")?;

        if conflicts::is_resolving(&project_repository) {
            bail!("cannot create a branch, project is in a conflicted state");
        }

        let mut semaphores = self.vbranch_semaphores.lock().await;
        let semaphore = semaphores
            .entry(project_id.to_string())
            .or_insert_with(|| Semaphore::new(1));
        let _permit = semaphore.acquire().await?;

        virtual_branches::create_virtual_branch(&gb_repository, create)?;
        Ok(())
    }

    pub async fn create_virtual_branch_from_branch(
        &self,
        project_id: &str,
        branch: &project_repository::branch::Name,
    ) -> Result<String> {
        let gb_repository = self.gb_repository(project_id)?;
        let project = self.gb_project(project_id)?;
        let project_repository = project_repository::Repository::open(&project)
            .context("failed to open project repository")?;

        let mut semaphores = self.vbranch_semaphores.lock().await;
        let semaphore = semaphores
            .entry(project_id.to_string())
            .or_insert_with(|| Semaphore::new(1));
        let _permit = semaphore.acquire().await?;

        let branch_id = virtual_branches::create_virtual_branch_from_branch(
            &gb_repository,
            &project_repository,
            branch,
            None,
        )?;
        // also apply the branch
        virtual_branches::apply_branch(&gb_repository, &project_repository, &branch_id)?;
        Ok(branch_id)
    }

    pub async fn update_virtual_branch(
        &self,
        project_id: &str,
        branch_update: virtual_branches::branch::BranchUpdateRequest,
    ) -> Result<()> {
        let gb_repository = self.gb_repository(project_id)?;

        let mut semaphores = self.vbranch_semaphores.lock().await;
        let semaphore = semaphores
            .entry(project_id.to_string())
            .or_insert_with(|| Semaphore::new(1));
        let _permit = semaphore.acquire().await?;

        virtual_branches::update_branch(&gb_repository, branch_update)?;
        Ok(())
    }

    pub async fn delete_virtual_branch(&self, project_id: &str, branch_id: &str) -> Result<()> {
        let gb_repository = self.gb_repository(project_id)?;

        let mut semaphores = self.vbranch_semaphores.lock().await;
        let semaphore = semaphores
            .entry(project_id.to_string())
            .or_insert_with(|| Semaphore::new(1));
        let _permit = semaphore.acquire().await?;

        virtual_branches::delete_branch(&gb_repository, branch_id)?;
        Ok(())
    }

    pub async fn unapply_virtual_branch(&self, project_id: &str, branch_id: &str) -> Result<()> {
        let gb_repository = self.gb_repository(project_id)?;
        let project = self.gb_project(project_id)?;
        let project_repository = project_repository::Repository::open(&project)
            .context("failed to open project repository")?;

        let mut semaphores = self.vbranch_semaphores.lock().await;
        let semaphore = semaphores
            .entry(project_id.to_string())
            .or_insert_with(|| Semaphore::new(1));
        let _permit = semaphore.acquire().await?;

        virtual_branches::unapply_branch(&gb_repository, &project_repository, branch_id)?;
        Ok(())
    }

    pub async fn apply_virtual_branch(&self, project_id: &str, branch_id: &str) -> Result<()> {
        let gb_repository = self.gb_repository(project_id)?;
        let project = self.gb_project(project_id)?;
        let project_repository = project_repository::Repository::open(&project)
            .context("failed to open project repository")?;

        let mut semaphores = self.vbranch_semaphores.lock().await;
        let semaphore = semaphores
            .entry(project_id.to_string())
            .or_insert_with(|| Semaphore::new(1));
        let _permit = semaphore.acquire().await?;

        virtual_branches::apply_branch(&gb_repository, &project_repository, branch_id)?;
        Ok(())
    }

    pub async fn commit_virtual_branch(
        &self,
        project_id: &str,
        branch: &str,
        message: &str,
    ) -> Result<()> {
        let gb_repository = self.gb_repository(project_id)?;
        let project = self.gb_project(project_id)?;
        let project_repository = project_repository::Repository::open(&project)
            .context("failed to open project repository")?;

        let mut semaphores = self.vbranch_semaphores.lock().await;
        let semaphore = semaphores
            .entry(project_id.to_string())
            .or_insert_with(|| Semaphore::new(1));
        let _permit = semaphore.acquire().await?;

        virtual_branches::commit(&gb_repository, &project_repository, branch, message)?;
        Ok(())
    }

    pub fn mark_resolved(&self, project_id: &str, path: &str) -> Result<()> {
        let project = self.gb_project(project_id)?;
        let project_repository = project_repository::Repository::open(&project)
            .context("failed to open project repository")?;
        // mark file as resolved
        conflicts::resolve(&project_repository, path)?;
        Ok(())
    }

    pub async fn push_virtual_branch(
        &self,
        project_id: &str,
        branch_id: &str,
    ) -> Result<(), Error> {
        let gb_repository = self.gb_repository(project_id).map_err(Error::Other)?;
        let project = self.gb_project(project_id).map_err(Error::Other)?;
        let project_repository =
            project_repository::Repository::open(&project).map_err(Error::Other)?;

        let mut semaphores = self.vbranch_semaphores.lock().await;
        let semaphore = semaphores
            .entry(project_id.to_string())
            .or_insert_with(|| Semaphore::new(1));
        let _permit = semaphore
            .acquire()
            .await
            .context("failed to acquire semaphore")
            .map_err(Error::Other)?;

        match virtual_branches::push(&project_repository, &gb_repository, branch_id) {
            Ok(_) => Ok(()),
            Err(virtual_branches::Error::UnsupportedAuthCredentials(_)) => Err(Error::Message(
                "unsupported authentication credentials".to_string(),
            )),
            Err(virtual_branches::Error::Other(e)) => Err(Error::Other(e)),
        }
    }

    pub fn fetch_from_target(&self, project_id: &str) -> Result<()> {
        let project = self.gb_project(project_id)?;
        let project_repository = project_repository::Repository::open(&project)?;
        project_repository
            .fetch()
            .context("failed to fetch from target")?;

        Ok(())
    }

    pub fn upsert_bookmark(&self, bookmark: &bookmarks::Bookmark) -> Result<()> {
        let gb_repository = self.gb_repository(&bookmark.project_id)?;
        let writer = bookmarks::Writer::new(&gb_repository).context("failed to open writer")?;
        writer.write(bookmark).context("failed to write bookmark")?;

        block_on({
            let bookmark = bookmark.clone();
            async move {
                if let Err(err) = self
                    .send_event(
                        &bookmark.project_id,
                        watcher::Event::Bookmark(bookmark.clone()),
                    )
                    .await
                {
                    log::error!("failed to send session event: {:#}", err);
                }
            }
        });

        Ok(())
    }

    pub fn list_bookmarks(
        &self,
        project_id: &str,
        range: Option<ops::Range<u128>>,
    ) -> Result<Vec<bookmarks::Bookmark>> {
        self.bookmarks_database
            .list_by_project_id(project_id, range)
    }

    pub fn list_session_deltas(
        &self,
        project_id: &str,
        session_id: &str,
        paths: Option<Vec<&str>>,
    ) -> Result<HashMap<String, Vec<deltas::Delta>>> {
        self.deltas_database
            .list_by_project_id_session_id(project_id, session_id, paths)
    }

    pub fn git_activity(
        &self,
        project_id: &str,
        start_time_ms: Option<u128>,
    ) -> Result<Vec<activity::Activity>> {
        let project = self.gb_project(project_id)?;
        let project_repository = project_repository::Repository::open(&project)
            .context("failed to open project repository")?;
        project_repository.git_activity(start_time_ms)
    }

    pub fn git_status(
        &self,
        project_id: &str,
    ) -> Result<HashMap<String, project_repository::FileStatus>> {
        let project = self.gb_project(project_id)?;
        let project_repository = project_repository::Repository::open(&project)
            .context("failed to open project repository")?;
        project_repository.git_status()
    }

    pub fn git_wd_diff(
        &self,
        project_id: &str,
        context_lines: u32,
    ) -> Result<HashMap<path::PathBuf, String>> {
        let project = self.gb_project(project_id)?;
        let project_repository = project_repository::Repository::open(&project)
            .context("failed to open project repository")?;

        let diff = diff::workdir(
            &project_repository,
            &project_repository.get_head()?.peel_to_commit()?.id(),
            &diff::Options { context_lines },
        )
        .context("failed to diff")?;

        let diff = diff
            .into_iter()
            .map(|(file_path, hunks)| {
                (
                    file_path,
                    hunks
                        .iter()
                        .map(|hunk| hunk.diff.to_string())
                        .collect::<Vec<_>>()
                        .join("\n"),
                )
            })
            .collect::<HashMap<_, _>>();

        Ok(diff)
    }

    pub fn git_match_paths(&self, project_id: &str, pattern: &str) -> Result<Vec<String>> {
        let project = self.gb_project(project_id)?;
        let project_repository = project_repository::Repository::open(&project)
            .context("failed to open project repository")?;
        project_repository.git_match_paths(pattern)
    }

    pub fn git_branches(&self, project_id: &str) -> Result<Vec<branch::LocalName>> {
        let project = self.gb_project(project_id)?;
        let project_repository = project_repository::Repository::open(&project)
            .context("failed to open project repository")?;
        project_repository.git_branches()
    }

    pub fn git_remote_branches(&self, project_id: &str) -> Result<Vec<branch::RemoteName>> {
        let project = self.gb_project(project_id)?;
        let project_repository = project_repository::Repository::open(&project)
            .context("failed to open project repository")?;
        project_repository.git_remote_branches()
    }

    pub fn git_remote_branches_data(
        &self,
        project_id: &str,
    ) -> Result<Vec<virtual_branches::RemoteBranch>> {
        let gb_repository = self.gb_repository(project_id)?;
        let project = self.gb_project(project_id)?;
        let project_repository = project_repository::Repository::open(&project)
            .context("failed to open project repository")?;
        virtual_branches::list_remote_branches(&gb_repository, &project_repository)
    }

    pub fn git_head(&self, project_id: &str) -> Result<String> {
        let project = self.gb_project(project_id)?;
        let project_repository = project_repository::Repository::open(&project)
            .context("failed to open project repository")?;
        let head = project_repository.get_head()?;
        Ok(head.name().unwrap().to_string())
    }

    pub fn git_set_config(&self, project_id: &str, key: &str, value: &str) -> Result<String> {
        let project = self.gb_project(project_id)?;
        let project_repository = project_repository::Repository::open(&project)
            .context("failed to open project repository")?;
        let repo = &project_repository.git_repository;
        let mut config = repo.config()?;
        config.open_level(git2::ConfigLevel::Local)?;
        config.set_str(key, value)?;
        Ok(value.to_string())
    }

    pub fn git_get_config(&self, project_id: &str, key: &str) -> Result<Option<String>> {
        let project = self.gb_project(project_id)?;
        let project_repository = project_repository::Repository::open(&project)?;
        let repo = &project_repository.git_repository;
        let config = repo.config()?;
        let value = config.get_string(key);
        match value {
            Ok(value) => Ok(Some(value)),
            Err(e) => {
                if e.code() == git2::ErrorCode::NotFound {
                    Ok(None)
                } else {
                    Err(e.into())
                }
            }
        }
    }

    pub fn git_set_global_config(&self, key: &str, value: &str) -> Result<String> {
        let mut config = git2::Config::open_default()?;
        config.set_str(key, value)?;
        Ok(value.to_string())
    }

    pub fn git_get_global_config(&self, key: &str) -> Result<Option<String>> {
        let config = git2::Config::open_default()?;
        let value = config.get_string(key);
        match value {
            Ok(value) => Ok(Some(value)),
            Err(e) => {
                if e.code() == git2::ErrorCode::NotFound {
                    Ok(None)
                } else {
                    Err(e.into())
                }
            }
        }
    }

    pub fn git_switch_branch(&self, project_id: &str, branch: &str) -> Result<()> {
        let project = self.gb_project(project_id)?;
        let project_repository = project_repository::Repository::open(&project)
            .context("failed to open project repository")?;
        let gb_repository = self.gb_repository(project_id)?;
        gb_repository.flush().context("failed to flush session")?;
        project_repository.git_switch_branch(branch)
    }

    pub fn git_gb_push(&self, project_id: &str) -> Result<()> {
        let gb_repository = self.gb_repository(project_id)?;
        gb_repository.push()
    }

    pub fn git_stage_files<P: AsRef<std::path::Path>>(
        &self,
        project_id: &str,
        paths: Vec<P>,
    ) -> Result<()> {
        let project = self.gb_project(project_id)?;
        let project_repository = project_repository::Repository::open(&project)
            .context("failed to open project repository")?;
        project_repository.git_stage_files(paths)
    }

    pub fn git_unstage_files<P: AsRef<std::path::Path>>(
        &self,
        project_id: &str,
        paths: Vec<P>,
    ) -> Result<()> {
        let project = self.gb_project(project_id)?;
        let project_repository = project_repository::Repository::open(&project)
            .context("failed to open project repository")?;
        project_repository.git_unstage_files(paths)
    }

    pub fn git_commit(&self, project_id: &str, message: &str, push: bool) -> Result<()> {
        let project = self.gb_project(project_id)?;
        let project_repository = project_repository::Repository::open(&project)
            .context("failed to open project repository")?;
        project_repository.git_commit(message, push)
    }

    pub fn search(&self, query: &search::Query) -> Result<search::Results> {
        self.searcher.search(query)
    }

    pub fn record_pty(&self, project_id: &str, typ: pty::Type, bytes: &[u8]) -> Result<()> {
        let gb_repository = self.gb_repository(project_id)?;
        let pty_writer = pty::Writer::new(&gb_repository)?;

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let record = pty::Record {
            timestamp,
            typ,
            bytes: bytes.to_vec(),
        };

        pty_writer.write(&record).context("failed to append pty")?;

        Ok(())
    }

    pub fn delete_all_data(&self) -> Result<()> {
        self.searcher
            .delete_all_data()
            .context("failed to delete search data")?;
        for project in self.list_projects()? {
            self.delete_project(&project.id)
                .context("failed to delete project")?;
        }
        Ok(())
    }
}
