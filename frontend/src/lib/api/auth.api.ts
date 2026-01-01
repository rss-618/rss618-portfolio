import { ok, err, type Result } from '$lib/utils';

const API_BASE = 'http://localhost:3000';

export async function login(email: string, password: string): Promise<Result> {
	try {
		const response = await fetch(`${API_BASE}/auth/login`, {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			credentials: 'include',
			body: JSON.stringify({ email, password }),
		});

		if (!response.ok) {
			return err('Invalid email or password');
		}

		return ok();
	} catch {
		return err('Network error');
	}
}

export async function logout(): Promise<void> {
	try {
		await fetch(`${API_BASE}/auth/logout`, {
			method: 'POST',
			credentials: 'include',
		});
	} catch {
		// Silently fail on logout network errors
	}
}
