<div align="center">
   <img align="center" width="128px" src="crates/gitbutler-tauri/icons/128x128@2x.png" />
	<h1 align="center"><b>GitButler</b></h1>
	<p align="center">
		Git Project Management tool, for collaborative development of projects using AI-assisted social branch-coding workflows
    <br />
    <a href="https://gitbutler.com"><strong>gitbutler.com »</strong></a>
    <br />
    <br />
    <b>Download for </b>
    macOS (<a href="https://app.gitbutler.com/downloads/release/darwin/aarch64/dmg">Apple Silicon</a> |
      <a href="https://app.gitbutler.com/downloads/release/darwin/x86_64/dmg">Intel</a>) ·
		Linux (<a href="https://app.gitbutler.com/downloads/release/linux/x86_64/gz">AppImage</a> |
       <a href="https://app.gitbutler.com/downloads/release/linux/x86_64/deb">deb</a>)
      ·
		Windows (<a href="https://app.gitbutler.com/downloads/release/windows/x86_64/msi">msi</a>)
    <br />
    <br />
    (Unstable Nightly releases can be found <a href="https://app.gitbutler.com/downloads">here</a>)
  </p>
</div>

<br/>

![gitbutler_client](https://github.com/gitbutlerapp/gitbutler/assets/16262535/0d180bfd-7a3c-4a66-a923-30534c5a8197)

[![CI][s0]][l0] [![BADGE][s6]][l6] [![TWEET][s1]][l1] [![DISCORD][s2]][l2] [![INSTA][s3]][l3] [![YOUTUBE][s5]][l5]

[s0]: https://github.com/gitbutlerapp/gitbutler/actions/workflows/push.yaml/badge.svg
[l0]: https://github.com/gitbutlerapp/gitbutler/actions/workflows/push.yaml
[s1]: https://img.shields.io/badge/Twitter-black?logo=x&logoColor=white
[l1]: https://twitter.com/intent/follow?screen_name=gitbutler
[s2]: https://img.shields.io/discord/1060193121130000425?label=Discord&color=5865F2
[l2]: https://discord.gg/MmFkmaJ42D
[s3]: https://img.shields.io/badge/Instagram-E4405F?logo=instagram&logoColor=white
[l3]: https://www.instagram.com/gitbutler/
[s5]: https://img.shields.io/youtube/channel/subscribers/UCEwkZIHGqsTGYvX8wgD0LoQ
[l5]: https://www.youtube.com/@gitbutlerapp
[s6]: https://img.shields.io/badge/GitButler-%23B9F4F2?logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPHN2ZyB3aWR0aD0iMzkiIGhlaWdodD0iMjgiIHZpZXdCb3g9IjAgMCAzOSAyOCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPHBhdGggZD0iTTI1LjIxNDUgMTIuMTk5N0wyLjg3MTA3IDEuMzg5MTJDMS41NDI5NSAwLjc0NjUzMiAwIDEuNzE0MDYgMCAzLjE4OTQ3VjI0LjgxMDVDMCAyNi4yODU5IDEuNTQyOTUgMjcuMjUzNSAyLjg3MTA3IDI2LjYxMDlMMjUuMjE0NSAxNS44MDAzQzI2LjcxOTcgMTUuMDcyMSAyNi43MTk3IDEyLjkyNzkgMjUuMjE0NSAxMi4xOTk3WiIgZmlsbD0iYmxhY2siLz4KPHBhdGggZD0iTTEzLjc4NTUgMTIuMTk5N0wzNi4xMjg5IDEuMzg5MTJDMzcuNDU3MSAwLjc0NjUzMiAzOSAxLjcxNDA2IDM5IDMuMTg5NDdWMjQuODEwNUMzOSAyNi4yODU5IDM3LjQ1NzEgMjcuMjUzNSAzNi4xMjg5IDI2LjYxMDlMMTMuNzg1NSAxNS44MDAzQzEyLjI4MDMgMTUuMDcyMSAxMi4yODAzIDEyLjkyNzkgMTMuNzg1NSAxMi4xOTk3WiIgZmlsbD0idXJsKCNwYWludDBfcmFkaWFsXzMxMF8xMjkpIi8%2BCjxkZWZzPgo8cmFkaWFsR3JhZGllbnQgaWQ9InBhaW50MF9yYWRpYWxfMzEwXzEyOSIgY3g9IjAiIGN5PSIwIiByPSIxIiBncmFkaWVudFVuaXRzPSJ1c2VyU3BhY2VPblVzZSIgZ3JhZGllbnRUcmFuc2Zvcm09InRyYW5zbGF0ZSgxNi41NzAxIDE0KSBzY2FsZSgxOS44NjQxIDE5LjgzODMpIj4KPHN0b3Agb2Zmc2V0PSIwLjMwMTA1NiIgc3RvcC1vcGFjaXR5PSIwIi8%2BCjxzdG9wIG9mZnNldD0iMSIvPgo8L3JhZGlhbEdyYWRpZW50Pgo8L2RlZnM%2BCjwvc3ZnPgo%3D
[l6]: https://gitbutler.com/

![Alt](https://repobeats.axiom.co/api/embed/fb23382bcf57c609832661874d3019a43555d6ae.svg 'Repobeats analytics for GitButler')

GitProjMgr is very much like GitButler, which is a git client that lets you work on multiple branches at the same time.
GitButler allows you to quickly organize file changes into separate branches while still having them applied to your working directory.
You can then push branches individually to your remote, or directly create pull requests.

In a nutshell, it's a more flexible version of `git add -p` and `git rebase -i`, allowing you to efficiently multitask across branches.

## How Does GitButleer (GB) / GitProjMgr (GPM) Work?

GB/GPM keep track of uncommitted changes in a layer on top of Git. Changes to files or parts of files can be grouped into what we call virtual branches. Whenever you are happy with the contents of a virtual branch, you can push it to a remote. GB/GPM makes sure that the state of other virtual branches is kept separate.

## How Do These Virtual Branches in GB or GPM Differ From Git Branches?

The branches that we know and love in Git are separate universes, and switching between them is a full context switch. GB/GPM allows you to work with multiple branches in parallel in the same working directory. This effectively means having the content of multiple branches available at the same time.

GitButler is aware of changes before they are committed. This allows it to keep a record of which virtual branch each individual diff belongs to. Effectively, this means that you can separate out individual branches with their content at any time to push them to a remote or to unapply them from your working directory.

And finally, while in Git it is preferable that you create your desired branch ahead of time, using GitButler you can move changes between virtual branches at any point during development.

## Why GitButler? Why GitProjMgr

We love Git and the work of [Scott Chacon](https://github.com/schacon), and not just [@schacon](https://github.com/schacon)'s [Pro Git](https://git-scm.com/book/en/v2) book, but Scott's work with languages and immersive training to speak/think as a native. We recognize that Git's porcelain user interface hasn't been fundamentally changed for 15 years, but we still believe that the data model architecture is basically sound and *certainly not going away any time soon*. While Git was originally written for Linux kernel devs sending patches to each other over mailing lists, most developers today have different workflows and needs and most people engaged in other creative pursuits and project management chores certainly have *extremely* different and diverse workflows *ON THE SURFACE*. But underneath, it still goes back the data model.

Instead of trying to fit the semantics of the Git command line interface into a graphical interface, the devs behind GitButler started with the branched developer workflow and mapped it back to Git. Our approach with GitProjMgr is to take this one step further and to make project management workflows AI-assisted, in order to be primarily code-free and to instead allow for social branch coding to be done primarily with natural language prompts.

## Tech

GitButler / GitProjMgr are very similar applications, at least on the surface and for the most part, under the hood, too. GitProjMgr will make choices for even lower latency and to facilitate the incorporation of AI-assisted natural language features for social branch coding and development/publishing of visual knowledgegraphs of conceptual ideas in a data visualization approach inspired by [arXiv labs](https://info.arxiv.org/labs/showcase.html), [SemanticScholar](https://www.semanticscholar.org/paper/Construction-of-the-Literature-Graph-in-Semantic-Ammar-Groeneveld/649def34f8be52c8b66281af98ae884c09aef38b) and [ConnectedPapers](https://www.connectedpapers.com/about). 

### Key Technologies for GitProjMgr
- **[Frontend](https://www.curated.design/articles/gitbutler)**: [Svelte](https://svelte.dev/), [TypeScript](https://www.typescriptlang.org), [TailwindCSS](https://github.com/tailwindlabs/tailwindcss)
- **Backend**: [Rust](https://www.rust-lang.org/), [Tauri](https://tauri.app/)
- **Data Storage**: Git-based distributed repositories
- **Synchronization**: Custom Git protocols, WebRTC
- **Cloud Infrastructure**: GCP/Azure/AWS for central repository
- **AI Integration**: LLM APIs for intelligent assistance
- **Authentication**: OAuth, JWT
- **DevOps**: GitHub Actions, Docker, k8s

## Documentation

You can find the GitButler end user documentation at: https://docs.gitbutler.com

The end user documentation for GitProjMgr is ... well ... *that might take a while ...*

## Bugs and Feature Requests

If you have a bug or feature request for GitButler, open an [issue](https://github.com/gitbutlerapp/gitbutler/issues/new),
or [join the GitButler Discord server](https://discord.gg/MmFkmaJ42D).

## AI Commit Message Generation

Commit message generation is an opt-in feature. You can enable it while adding your repository for the first time or later in the project settings.

Currently, GitButler uses OpenAI's API for diff summarization, which means that if enabled, code diffs would be sent to OpenAI's servers.

Our goal is to make this feature more modular such that in the future you can modify the prompt as well as plug a different LLM endpoints (including local ones).

## Contributing

So you want to help out? Please check out the [CONTRIBUTING.md](CONTRIBUTING.md)
document.

If you want to skip right to getting the code to actually compile, take a look
at the [DEVELOPMENT.md](DEVELOPMENT.md) file.

## Main Features of GB / GPM

- **Virtual Branches**
  - Organize work on multiple branches simultaneously, rather than constantly switching branches
  - Automatically create new branches when needed
- **Easy Commit Management**
  - Undo, Amend and Squash commits by dragging and dropping
- **Undo Timeline**
  - Logs all operations and changes and allows you to easily undo or revert any operation
- **GitHub Integration**
  - Authenticate to GitHub to open Pull Requests, list branches and statuses and more
- **Easy SSH Key Management**
  - GitButler/GitProjMgr can generate an SSH key to upload to GitHub automatically
- **AI Tooling**
  - Automatically write commit messages based on your work in progress
  - Automatically create descriptive branch names
- **Commit Signing**
  - Easy commit signing with GPG or SSH

## Example Git Project Management Roadmap

### Start With A 100-Point Plan for Launching A Venture

For discussion purposes, we will consider something like nursery business. Plans are cheap, plans are easy, plans don't matter EXCEPT that the process of planning, iteratively and working the plan is what it's about.  Obviously, no plan ever survives confrontation with Reality, but that is the point. It's not the plan, it's the **importance of the PLANNING process is to THOROUGHLY visualize success and everything that is going to have to happen; that why we plan forward and backward, but when we decide to get serious, we start at the end and work backward from what has to happen to ensure success.**

*Project PLANNING is about PROGRAMMING your mind to be successful* ... before you commit capital or resources or your time, you have to run the program in your mind ... most ventures will not work, plans will never go anywhere -- but we are better for the process of having investigated in new plan AS LONG AS WE HAVE NOT COMMITTED THE CAPITAL, RESOURCES, TIME and ENERGY to the idea.

Obviously, your plans, that you do decide to actually go forward with will change as you launch and get into it ... so the whole point of making project planning easier with an automated project mgmt tools is it will be necessary to always be re-factoring the plan, with minor adjustments ... because, once capital is committed, most of the hard effort, sweat and frustration will have to be spent on the actual work necessary to guide the venture to success ... you do not want to start the planning process AFTER you've thrown way too much money at something that didn't have a plan.  

#### Phase 1: [Research & Planning](https://docs.google.com/document/d/1fF9RR1BlY3QMfvxpc1vhcfP8yrUVoy_P5XEsoYFnTZU/edit?usp=sharing)

##### Market Research
1. Conduct local market research to identify demand for plants in your area
2. Research competitors (other nurseries, garden centers, big box stores)
3. Identify your target customer demographic and their needs
4. Analyze seasonal buying patterns in your region
5. Determine which plant types have the highest margins and demand

##### Business Foundation
6. Define your nursery's unique selling proposition (specialization in natives, rare plants, etc.)
7. Create a detailed business plan with 3-5 year projections
8. Determine your initial startup budget and funding sources
9. Select a business structure (sole proprietorship, LLC, etc.)
10. Register your business name and obtain necessary licenses

##### Location & Layout
11. Find the optimal location considering climate, visibility, and zoning
12. Calculate space requirements for growing areas, retail, parking
13. Design efficient layout for water access and customer flow
14. Determine whether to lease or purchase property
15. Plan for potential future expansion

##### Plant Selection Strategy
16. Research which plants grow well in your climate zone
17. Decide on plant categories to offer (perennials, trees, shrubs, etc.)
18. Identify specialty niches that may be underserved
19. Plan your inventory mix by season
20. Create a sourcing strategy for initial stock

#### Phase 2: Setup & Infrastructure 

##### Physical Infrastructure
21. Develop site plan and obtain necessary permits
22. Install irrigation systems and water management
23. Build or purchase greenhouse structures
24. Create shade structures for sensitive plants
25. Establish propagation and growing areas
26. Construct retail/customer spaces and pathways
27. Build secure storage for equipment and supplies
28. Install proper drainage systems
29. Set up utilities (water, electricity, internet)
30. Create signage and wayfinding for customers

##### Equipment & Supplies
31. Purchase essential growing equipment (potting benches, etc.)
32. Acquire vehicles for transport and deliveries
33. Source bulk soil, amendments, and growing media
34. Purchase containers, pots, and plant labels
35. Obtain hand tools and power equipment
36. Set up POS system and inventory management software
37. Purchase safety equipment and first aid supplies
38. Source environmentally friendly packaging materials
39. Acquire plant protection supplies (frost cloth, etc.)
40. Set up office equipment and supplies

#### Phase 3: [Operations & Systems](https://docs.google.com/document/d/1zwxpLprds4NznpOnJF2ku6GE1ggPvpSp47mYXBLoYmo/edit?usp=sharing)

##### Production Planning
41. Create detailed growing schedules by season
42. Establish propagation protocols for different plant types
43. Develop pest and disease management plans
44. Create standard operating procedures for plant care
45. Establish quality control standards and processes

##### Business Operations
46. Set up accounting and bookkeeping systems
47. Develop pricing strategies for different plant categories
48. Create employee roles and responsibilities documentation
49. Establish inventory management procedures
50. Develop customer service policies
51. Create sales and checkout procedures
52. Set up supplier relationships and ordering processes
53. Establish security protocols for the property
54. Create maintenance schedules for equipment and facilities
55. Develop waste management and recycling systems

##### Legal & Compliance
56. Obtain necessary business insurance (liability, property, etc.)
57. Understand agricultural regulations in your area
58. Comply with plant sale regulations and restrictions
59. Set up tax collection systems
60. Create safety protocols and employee training

#### Phase 4: [Marketing & Customer Acquisition](https://docs.google.com/document/d/1hDgQJxklhzQqpygYXvjVCScDun3ydm-Y6n6PRPQij9A/edit?usp=sharing)

##### Brand Development
61. Design professional logo and brand identity
62. Create consistent visual standards for all materials
63. Develop your brand story and mission statement
64. Build a user-friendly website with plant inventory
65. Create business profiles on relevant social media platforms

##### Marketing Strategies
66. Develop a year-round marketing calendar
67. Create email marketing program and build subscriber list
68. Plan seasonal promotions and sales events
69. Develop content marketing strategy (plant care guides, etc.)
70. Build relationships with local landscapers and designers

##### Customer Experience
71. Create educational materials for customers
72. Design plant care information tags and materials
73. Develop a customer loyalty program
74. Plan workshops and events to build community
75. Create a pleasant shopping environment with displays
76. Train staff on plant knowledge and customer service
77. Establish a customer feedback system
78. Create policies for plant guarantees or warranties
79. Design plant combination suggestions for shoppers
80. Develop online ordering and pickup/delivery options

#### [Phase 5: Growth & Sustainability](https://docs.google.com/document/d/1XtyHvfCBV1Z2qddu4lJOdC6o7Zcv-xhkQpS2QlbWxBc/edit?usp=sharing) 

##### Business Growth
81. Establish metrics to track business performance
82. Create benchmarks for profitability by department
83. Develop strategies for increasing average transaction value
84. Plan for seasonal cash flow management
85. Explore additional revenue streams (workshops, design services)
86. Build wholesale relationships with landscapers
87. Explore potential for mail-order business expansion
88. Create a referral program for existing customers
89. Develop cross-promotion opportunities with complementary businesses
90. Plan for strategic equipment upgrades and investments

##### Sustainability & Future Planning
91. Implement water conservation practices
92. Develop organic and sustainable growing practices
93. Create a plan to minimize plastic use and waste
94. Establish energy efficiency measures
95. Build educational programs around native plants and sustainability
96. Plan for staff development and advancement
97. Create disaster and emergency preparedness plans
98. Develop succession planning for business continuity
99. Build community partnerships and involvement
100. Create a long-term vision for business evolution and growth

As any comprehensive business plan illustrates, **the entreprenur must START by looking at the end and reading backwards adjusting and re-adjusting, gradually adding more and more depth ... then refining. revising, refactoring, rethinking, redoing the plan AND POSSIBLY START COMPLETELY OVER ... the planning process ensures that capital is not squandered by just jumping in and doing something for the sake of getting something going. 

In the final analysis, GitProjMgr is ONLY a tool -- the project management actually consists of doing the work. ***It's like a fitness app*** ... the fitness app does not do the important repititions for the person; the point of a fitness app is to spend less time searching for data such as finding new WODs and free the mind from tracking metrics ... in order to focus on doing quality repititions. 

