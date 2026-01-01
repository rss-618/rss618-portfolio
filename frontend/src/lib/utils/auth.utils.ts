import type { FirebaseAuthUser, JwtClaims, JwtToken } from '$lib/dto';

export function parseJwtToken(token: string): JwtToken | null {
	const segments = token.split('.');
	if (segments.length !== 3) return null;

	const [header, claimsBase64, signature] = segments;

	try {
		const claims = JSON.parse(atob(claimsBase64)) as JwtClaims;
		return { header, claims, signature };
	} catch {
		return null;
	}
}

export function getUserFromJwtToken(idToken: string): FirebaseAuthUser | null {
	const jwt = parseJwtToken(idToken);
	if (!jwt) return null;

	return {
		uid: jwt.claims.user_id || jwt.claims.sub,
		email: jwt.claims.email,
	};
}

export function isJwtTokenExpired(idToken: string): boolean {
	const jwt = parseJwtToken(idToken);
	if (!jwt) return true;

	const expiresAt = jwt.claims.exp * 1000;
	const bufferMs = 5 * 60 * 1000; // 5 minutes buffer
	return Date.now() > expiresAt - bufferMs;
}
