import type { Handle } from '@sveltejs/kit';
import { THEME_COOKIE_NAME, THEME_ATTRIBUTE, DEFAULT_THEME } from '$lib/constants';
import type { Theme } from '$lib/types';

const VALID_THEMES: Theme[] = ['light', 'dark'];

export const handle: Handle = async ({ event, resolve }) => {
	const themeCookie = event.cookies.get(THEME_COOKIE_NAME) as Theme | undefined;
	const theme = themeCookie && VALID_THEMES.includes(themeCookie) ? themeCookie : DEFAULT_THEME;

	return resolve(event, {
		transformPageChunk: ({ html }) => {
			return html.replace(`${THEME_ATTRIBUTE}=""`, `${THEME_ATTRIBUTE}="${theme}"`);
		},
	});
};
