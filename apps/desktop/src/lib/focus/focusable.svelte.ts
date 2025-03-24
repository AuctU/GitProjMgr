import { FocusManager } from '$lib/focus/focusManager.svelte';
import { getContext } from '@gitbutler/shared/context';
import { on } from 'svelte/events';
import type { Action } from 'svelte/action';

interface FocusableOptions {
	id: string;
	parentId?: string | null;
}
/**
 * Svelte action that registers an element as a focusable area.
 */
// eslint-disable-next-line func-style
export const focusable: Action<HTMLElement, FocusableOptions> = (element, options) => {
	const { id, parentId = null } = options;
	const focus = getContext(FocusManager);

	$effect(() => {
		const unlistenFocus = on(element, 'focus', onFocus);
		const unlistenBlur = on(element, 'blur', onBlur);
		focus.register(id, parentId, element);
		return () => {
			focus.unregister(id);
			unlistenFocus();
			unlistenBlur();
		};
	});

	function onFocus() {
		element.classList.add('focused');
	}

	function onBlur() {
		element.classList.remove('focused');
	}

	function handleClick(event: MouseEvent) {
		focus.setActive(id);
		event.stopPropagation();
	}

	function handleKeydown(event: KeyboardEvent) {
		event.stopPropagation();
		if (event.key === 'Tab') {
			event.preventDefault();
			focus.focusSibling(!event.shiftKey);
		} else if (event.metaKey && event.key === 'ArrowUp') {
			event.preventDefault();
			focus.focusParent();
		} else if (event.metaKey && event.key === 'ArrowDown') {
			event.preventDefault();
			focus.focusFirstChild();
		}
	}

	const unlistenKeyDown = on(element, 'keydown', handleKeydown);
	const unlistenClick = on(element, 'click', handleClick);
	element.tabIndex = 0;

	return {
		destroy() {
			focus.unregister(id);
			unlistenKeyDown();
			unlistenClick();
		}
	};
};
