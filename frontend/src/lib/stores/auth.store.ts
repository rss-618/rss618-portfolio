import { writable, get } from 'svelte/store';
import { browser } from '$app/environment';
import { signInWithEmailAndPassword, signOut, onAuthStateChanged, type User } from 'firebase/auth';
import { getFirebaseAuth } from '$lib/services/firebase';

export interface AuthState {
	user: User | null;
	loading: boolean;
	error: string | null;
}

const initialState: AuthState = {
	user: null,
	loading: true,
	error: null,
};

function createAuthStore() {
	const store = writable<AuthState>(initialState);
	let unsubscribe: (() => void) | null = null;

	function init(): void {
		if (!browser || unsubscribe) return;

		const firebaseAuth = getFirebaseAuth();
		unsubscribe = onAuthStateChanged(
			firebaseAuth,
			(user) => {
				store.set({ user, loading: false, error: null });
			},
			() => {
				store.set({ user: null, loading: false, error: 'Authentication error' });
			}
		);
	}

	async function login(email: string, password: string): Promise<{ success: boolean; error?: string }> {
		if (!browser) return { success: false, error: 'Login failed' };

		store.update((state) => ({ ...state, loading: true, error: null }));

		try {
			const firebaseAuth = getFirebaseAuth();
			await signInWithEmailAndPassword(firebaseAuth, email, password);
			return { success: true };
		} catch {
			const message = 'Invalid email or password';
			store.update((state) => ({ ...state, loading: false, error: message }));
			return { success: false, error: message };
		}
	}

	async function logout(): Promise<void> {
		if (!browser) return;

		try {
			const firebaseAuth = getFirebaseAuth();
			await signOut(firebaseAuth);
		} catch {
			store.update((state) => ({ ...state, error: 'Logout failed' }));
		}
	}

	function clearError(): void {
		store.update((state) => ({ ...state, error: null }));
	}

	function isAuthenticated(): boolean {
		return get(store).user !== null;
	}

	return {
		subscribe: store.subscribe,
		init,
		login,
		logout,
		clearError,
		isAuthenticated,
	};
}

export const auth = createAuthStore();

// Auto-initialize in browser
if (browser) {
	auth.init();
}
