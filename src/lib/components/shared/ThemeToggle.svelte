<script lang="ts">
	import { theme } from '$lib/stores/theme.store';
	import IconSun from '$lib/assets/icons/IconSun.svelte';
	import IconMoon from '$lib/assets/icons/IconMoon.svelte';
</script>

<button onclick={() => theme.toggle()} class="theme-toggle" aria-label="Toggle theme">
	<span class="icon-sun"><IconSun /></span>
	<span class="icon-moon"><IconMoon /></span>
</button>

<style>
	.theme-toggle {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 2.5rem;
		height: 2.5rem;
		padding: 0;
		background-color: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-md);
		color: var(--color-text);
		cursor: pointer;
		transition:
			background-color var(--transition-fast),
			border-color var(--transition-fast),
			color var(--transition-fast);
	}

	.theme-toggle:hover {
		background-color: var(--color-border);
		color: var(--color-accent);
	}

	.icon-sun,
	.icon-moon {
		display: flex;
		align-items: center;
		justify-content: center;
	}

	/* Dark mode: show sun (to switch to light), hide moon */
	:global([data-theme='dark']) .icon-sun {
		display: flex;
	}
	:global([data-theme='dark']) .icon-moon {
		display: none;
	}

	/* Light mode: show moon (to switch to dark), hide sun */
	:global([data-theme='light']) .icon-sun {
		display: none;
	}
	:global([data-theme='light']) .icon-moon {
		display: flex;
	}
</style>
