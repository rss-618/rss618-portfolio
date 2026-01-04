import { createClient, ConnectError, Code } from '@connectrpc/connect';
import { AuthService } from '$proto/auth_pb';
import { ok, err, type Result } from '$lib/utils';
import { transport } from './transport';

const client = createClient(AuthService, transport);

export async function login(email: string, password: string): Promise<Result> {
	try {
		await client.login({ email, password });
		return ok();
	} catch (e) {
		if (
			e instanceof ConnectError &&
			(e.code === Code.Unauthenticated || e.code === Code.PermissionDenied)
		) {
			return err('Invalid email or password');
		} else {
			return err('Network error');
		}
	}
}

export async function logout(): Promise<void> {
	try {
		await client.logout({});
	} catch {
		// Silently fail on logout network errors
	}
}
