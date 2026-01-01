<script lang="ts">
	import '$lib/styles/global.css';
	import favicon from '$lib/assets/favicon.svg';
	import NavBar from '$lib/components/shared/NavBar.svelte';
	import Footer from '$lib/components/shared/Footer.svelte';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="app">
	<NavBar />

	<main>
		{@render children()}
	</main>

	<Footer />
</div>

<style>
	.app {
		display: flex;
		flex-direction: column;
		min-height: 100vh;
	}

	main {
		flex: 1;
		padding: var(--space-xl);
	}

	@media (max-width: 768px) {
		main {
			padding: var(--space-md);
		}
	}
</style>
