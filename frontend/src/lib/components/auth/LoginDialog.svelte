<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { login } from '$lib/api/auth.api';

	interface Props {
		open: boolean;
		onClose: () => void;
		onSuccess: () => void;
	}

	let { open, onClose, onSuccess }: Props = $props();

	let dialogRef: HTMLDialogElement;
	let email = $state('');
	let password = $state('');
	let isSubmitting = $state(false);
	let errorMessage = $state('');

	$effect(() => {
		if (open) {
			dialogRef?.showModal();
		} else {
			dialogRef?.close();
		}
	});

	function handleBackdropClick(event: MouseEvent) {
		if (event.target === dialogRef) {
			handleClose();
		}
	}

	function handleClose() {
		email = '';
		password = '';
		errorMessage = '';
		onClose();
	}

	async function handleSubmit(event: SubmitEvent) {
		event.preventDefault();
		if (isSubmitting) return;

		isSubmitting = true;
		errorMessage = '';

		const result = await login(email, password);

		isSubmitting = false;

		if (result.success) {
			email = '';
			password = '';
			await invalidateAll();
			onSuccess();
		} else {
			errorMessage = result.error;
		}
	}
</script>

<dialog bind:this={dialogRef} class="login-dialog" onclick={handleBackdropClick}>
	<div class="dialog-content">
		<h2>Login</h2>

		<form onsubmit={handleSubmit}>
			<div class="field">
				<label for="email">Email</label>
				<input
					id="email"
					type="email"
					bind:value={email}
					required
					autocomplete="email"
					disabled={isSubmitting}
				/>
			</div>

			<div class="field">
				<label for="password">Password</label>
				<input
					id="password"
					type="password"
					bind:value={password}
					required
					autocomplete="current-password"
					disabled={isSubmitting}
				/>
			</div>

			{#if errorMessage}
				<p class="error">{errorMessage}</p>
			{/if}

			<div class="actions">
				<button type="button" class="cancel-btn" onclick={handleClose} disabled={isSubmitting}>
					Cancel
				</button>
				<button type="submit" class="submit-btn" disabled={isSubmitting}>
					{isSubmitting ? 'Logging in...' : 'Login'}
				</button>
			</div>
		</form>
	</div>
</dialog>

<style>
	.login-dialog {
		position: fixed;
		inset: 0;
		width: 100%;
		max-width: 100%;
		height: 100%;
		max-height: 100%;
		margin: 0;
		padding: 0;
		border: none;
		background: transparent;
	}

	.login-dialog[open] {
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.login-dialog::backdrop {
		background-color: rgba(0, 0, 0, 0.5);
		animation: fadeIn var(--transition-base) ease;
	}

	.dialog-content {
		background-color: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-md);
		padding: var(--space-xl);
		width: min(400px, 90vw);
		animation: scaleIn var(--transition-base) ease;
	}

	h2 {
		margin: 0 0 var(--space-lg);
		font-size: 1.5rem;
		color: var(--color-text);
	}

	.field {
		margin-bottom: var(--space-md);
	}

	label {
		display: block;
		margin-bottom: var(--space-xs);
		font-size: 0.875rem;
		color: var(--color-text-muted);
	}

	input {
		width: 100%;
		padding: var(--space-sm) var(--space-md);
		background-color: var(--color-bg);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-sm);
		color: var(--color-text);
		font-size: 1rem;
		transition:
			border-color var(--transition-fast),
			box-shadow var(--transition-fast);
	}

	input:focus {
		outline: none;
		border-color: var(--color-accent);
		box-shadow: 0 0 0 2px color-mix(in srgb, var(--color-accent) 20%, transparent);
	}

	input:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.error {
		margin: var(--space-md) 0;
		padding: var(--space-sm) var(--space-md);
		background-color: color-mix(in srgb, var(--color-error) 10%, transparent);
		border: 1px solid color-mix(in srgb, var(--color-error) 30%, transparent);
		border-radius: var(--radius-sm);
		color: var(--color-error);
		font-size: 0.875rem;
	}

	.actions {
		display: flex;
		justify-content: flex-end;
		gap: var(--space-sm);
		margin-top: var(--space-lg);
	}

	button {
		padding: var(--space-sm) var(--space-lg);
		border-radius: var(--radius-sm);
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition:
			background-color var(--transition-fast),
			border-color var(--transition-fast),
			opacity var(--transition-fast);
	}

	button:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.cancel-btn {
		background-color: transparent;
		border: 1px solid var(--color-border);
		color: var(--color-text-muted);
	}

	.cancel-btn:hover:not(:disabled) {
		border-color: var(--color-text-muted);
		color: var(--color-text);
	}

	.submit-btn {
		background-color: var(--color-accent);
		border: 1px solid var(--color-accent);
		color: var(--color-bg);
	}

	.submit-btn:hover:not(:disabled) {
		background-color: var(--color-accent-hover);
		border-color: var(--color-accent-hover);
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	@keyframes scaleIn {
		from {
			opacity: 0;
			transform: scale(0.95);
		}
		to {
			opacity: 1;
			transform: scale(1);
		}
	}
</style>
