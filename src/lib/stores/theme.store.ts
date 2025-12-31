import { writable, get } from 'svelte/store';
import { browser } from '$app/environment';
import { THEME_COOKIE_NAME, THEME_ATTRIBUTE, DEFAULT_THEME } from '$lib/constants';
import type { Theme } from '$lib/types';

function getInitialTheme(): Theme {
	if (!browser) return DEFAULT_THEME;

	const attr = document.documentElement.getAttribute(THEME_ATTRIBUTE);
	if (attr === 'light' || attr === 'dark') return attr;

	return DEFAULT_THEME;
}

function setThemeCookie(theme: Theme): void {
	if (!browser) return;
	document.cookie = `${THEME_COOKIE_NAME}=${theme};path=/;max-age=31536000;SameSite=Lax`;
}

function applyTheme(theme: Theme): void {
	if (!browser) return;
	document.documentElement.setAttribute(THEME_ATTRIBUTE, theme);
}

function createThemeStore() {
	const store = writable<Theme>(getInitialTheme());

	function set(theme: Theme): void {
		setThemeCookie(theme);
		applyTheme(theme);
		store.set(theme);
	}

	function toggle(): void {
		const current = get(store);
		set(current === 'dark' ? 'light' : 'dark');
	}

	return {
		subscribe: store.subscribe,
		set,
		toggle,
	};
}

export const theme = createThemeStore();
