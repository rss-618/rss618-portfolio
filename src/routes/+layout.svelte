<script lang="ts">
	import '$lib/styles/global.css';
	import favicon from '$lib/assets/favicon.svg';
	import ThemeToggle from '$lib/components/shared/ThemeToggle.svelte';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<header>
	<ThemeToggle />
</header>

<main>
	{@render children()}
</main>

<style>
	header {
		display: flex;
		justify-content: flex-end;
		padding: var(--space-md);
	}

	main {
		padding: var(--space-md);
	}
</style>
