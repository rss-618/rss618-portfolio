import { PUBLIC_API_URL } from '$env/static/public';
import { createClient, type Interceptor, ConnectError, Code } from '@connectrpc/connect';
import { createGrpcWebTransport } from '@connectrpc/connect-web';
import { AuthService } from '$proto/auth_pb';

// Track if a refresh is in progress to prevent concurrent refreshes
let refreshPromise: Promise<void> | null = null;

async function refreshToken(): Promise<void> {
	// If already refreshing, wait for existing refresh
	if (refreshPromise) {
		return refreshPromise;
	}

	refreshPromise = (async () => {
		try {
			const authClient = createClient(AuthService, baseTransport);
			await authClient.refreshToken({});
		} finally {
			refreshPromise = null;
		}
	})();

	return refreshPromise;
}

/**
 * Interceptor that automatically refreshes the token on UNAUTHENTICATED errors.
 * Retries the request once after a successful refresh.
 */
const authRefreshInterceptor: Interceptor = (next) => async (req) => {
	try {
		return await next(req);
	} catch (error) {
		if (error instanceof ConnectError && error.code === Code.Unauthenticated) {
			try {
				await refreshToken();
				return await next(req);
			} catch {
				// Refresh failed, throw original error
				throw error;
			}
		}
		throw error;
	}
};

const fetchWithCredentials = (input: RequestInfo | URL, init?: RequestInit) =>
	fetch(input, { ...init, credentials: 'include' });

const baseTransport = createGrpcWebTransport({
	baseUrl: PUBLIC_API_URL,
	fetch: fetchWithCredentials,
});

export const transport = createGrpcWebTransport({
	baseUrl: PUBLIC_API_URL,
	fetch: fetchWithCredentials,
	interceptors: [authRefreshInterceptor],
});
