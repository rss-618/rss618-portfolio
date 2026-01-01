<script lang="ts">
	import '$lib/styles/global.css';
	import '$lib/services/firebase';
	import NavBar from '$lib/components/shared/NavBar.svelte';
	import Footer from '$lib/components/shared/Footer.svelte';
	import AuthProvider from '$lib/components/auth/AuthProvider.svelte';

	let { children } = $props();
</script>

<div class="app">
	<NavBar />

	<main>
		{@render children()}
	</main>

	<Footer />
</div>

<AuthProvider />

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
