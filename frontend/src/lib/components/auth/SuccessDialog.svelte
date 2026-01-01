<script lang="ts">
	interface Props {
		open: boolean;
		onClose: () => void;
	}

	let { open, onClose }: Props = $props();

	let dialogRef: HTMLDialogElement;

	$effect(() => {
		if (open) {
			dialogRef?.showModal();
		} else {
			dialogRef?.close();
		}
	});

	function handleBackdropClick(event: MouseEvent) {
		if (event.target === dialogRef) {
			onClose();
		}
	}
</script>

<dialog bind:this={dialogRef} class="success-dialog" onclick={handleBackdropClick}>
	<div class="dialog-content">
		<div class="icon-container">
			<svg class="check-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<path d="M20 6L9 17l-5-5" stroke-linecap="round" stroke-linejoin="round" />
			</svg>
		</div>

		<h2>Login Successful</h2>
		<p>You are now logged in.</p>

		<button class="close-btn" onclick={onClose}>Close</button>
	</div>
</dialog>

<style>
	.success-dialog {
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

	.success-dialog[open] {
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.success-dialog::backdrop {
		background-color: rgba(0, 0, 0, 0.5);
		animation: fadeIn var(--transition-base) ease;
	}

	.dialog-content {
		background-color: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-md);
		padding: var(--space-xl);
		width: min(320px, 90vw);
		text-align: center;
		animation: scaleIn var(--transition-base) ease;
	}

	.icon-container {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 3rem;
		height: 3rem;
		margin: 0 auto var(--space-md);
		background-color: color-mix(in srgb, var(--color-success) 15%, transparent);
		border-radius: 50%;
	}

	.check-icon {
		width: 1.5rem;
		height: 1.5rem;
		color: var(--color-success);
	}

	h2 {
		margin: 0 0 var(--space-xs);
		font-size: 1.25rem;
		color: var(--color-text);
	}

	p {
		margin: 0 0 var(--space-lg);
		color: var(--color-text-muted);
		font-size: 0.875rem;
	}

	.close-btn {
		padding: var(--space-sm) var(--space-xl);
		background-color: var(--color-accent);
		border: 1px solid var(--color-accent);
		border-radius: var(--radius-sm);
		color: var(--color-bg);
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition:
			background-color var(--transition-fast),
			border-color var(--transition-fast);
	}

	.close-btn:hover {
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
