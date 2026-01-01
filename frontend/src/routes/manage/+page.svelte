<script lang="ts">
	import { error } from '@sveltejs/kit';
	import { auth } from '$lib/stores/auth.store';
	import ManagePage from '$lib/components/manage/ManagePage.svelte';

	$effect(() => {
		if (!$auth.loading && !$auth.user) {
			error(404, 'Not found');
		}
	});
</script>

{#if $auth.loading}
	<div class="loading">Loading...</div>
{:else if $auth.user}
	<ManagePage />
{/if}

<style>
	.loading {
		display: flex;
		align-items: center;
		justify-content: center;
		min-height: 50vh;
		color: var(--color-text-muted);
	}
</style>
