import type { JwtClaims } from './jwt-claims';

/**
 * Decoded JWT with typed claims
 */
export type JwtToken = {
	header: string;
	claims: JwtClaims;
	signature: string;
}
