<script lang="ts">
	import { goto } from '$app/navigation';
	import { auth } from '$lib/stores/auth.store';
	import { Routes } from '$lib/constants';

	async function handleLogout() {
		await auth.logout();
		goto(Routes.HOME);
	}
</script>

<div class="manage-page">
	<header class="page-header">
		<h1>Manage</h1>
		<button class="logout-btn" onclick={handleLogout}>Logout</button>
	</header>

	<section class="content">
		<p>Welcome to the admin area.</p>
	</section>
</div>

<style>
	.manage-page {
		max-width: 800px;
		margin: 0 auto;
	}

	.page-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: var(--space-xl);
	}

	h1 {
		margin: 0;
		font-size: 2rem;
		color: var(--color-text);
	}

	.logout-btn {
		padding: var(--space-sm) var(--space-lg);
		background-color: transparent;
		border: 1px solid var(--color-border);
		border-radius: var(--radius-sm);
		color: var(--color-text-muted);
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition:
			background-color var(--transition-fast),
			border-color var(--transition-fast),
			color var(--transition-fast);
	}

	.logout-btn:hover {
		background-color: var(--color-surface);
		border-color: var(--color-text-muted);
		color: var(--color-text);
	}

	.content {
		color: var(--color-text-muted);
	}
</style>
