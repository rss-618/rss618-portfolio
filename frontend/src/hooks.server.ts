import type { Handle, RequestEvent } from '@sveltejs/kit';
import { THEME_COOKIE_NAME, THEME_ATTRIBUTE, DEFAULT_THEME } from '$lib/constants';
import type { Theme } from '$lib/types';
import type { FirebaseAuthUser } from '$lib/dto';
import { getUserFromJwtToken } from '$lib/utils';

const VALID_THEMES: Theme[] = ['light', 'dark'];

let cachedToken: string | null = null;
let cachedUser: FirebaseAuthUser | null = null;

function getTheme(event: RequestEvent): Theme {
	const themeCookie = event.cookies.get(THEME_COOKIE_NAME) as Theme | undefined;
	return themeCookie && VALID_THEMES.includes(themeCookie) ? themeCookie : DEFAULT_THEME;
}

function getUser(event: RequestEvent): FirebaseAuthUser | null {
	const idToken = event.cookies.get('id_token') ?? null;

	if (idToken !== cachedToken) {
		cachedToken = idToken;
		cachedUser = idToken ? getUserFromJwtToken(idToken) : null;
	}

	return cachedUser;
}

export const handle: Handle = async ({ event, resolve }) => {
	const theme = getTheme(event);
	event.locals.user = getUser(event);

	return resolve(event, {
		transformPageChunk: ({ html }) => {
			return html.replace(`${THEME_ATTRIBUTE}=""`, `${THEME_ATTRIBUTE}="${theme}"`);
		},
	});
};
