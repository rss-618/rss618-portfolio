export type Result<T = void, E = string> =
	| { success: true; data: T }
	| { success: false; error: E };

export function ok(): Result<void, never>;
export function ok<T>(data: T): Result<T, never>;
export function ok<T>(data?: T): Result<T | void, never> {
	return { success: true, data: data as T | void };
}

export function err<E>(error: E): Result<never, E> {
	return { success: false, error };
}
