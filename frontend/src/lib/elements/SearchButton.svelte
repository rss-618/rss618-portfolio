<script lang="ts">
	import { tick } from 'svelte';
	import SearchIcon from '$lib/assets/icons/SearchIcon.svelte';

	interface Props {
		placeholder?: string;
		onSearch: (query: string) => void;
	}

	let { placeholder = 'Search...', onSearch }: Props = $props();

	let query = $state('');
	let expanded = $state(false);
	let inputRef = $state<HTMLInputElement | null>(null);

	function handleSearch() {
		onSearch(query);
	}

	function handleKeyDown(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			handleSearch();
		} else if (e.key === 'Escape') {
			collapse();
		}
	}

	async function expand() {
		expanded = true;
		await tick();
		inputRef?.focus();
	}

	function collapse() {
		if (!query) {
			expanded = false;
		}
	}
</script>

<div class="search-container" class:expanded>
	<input
		type="text"
		class="search-input"
		{placeholder}
		bind:value={query}
		bind:this={inputRef}
		onkeydown={handleKeyDown}
		onblur={collapse}
		onfocus={expand}
	/>
	<button
		class="search-btn"
		onmousedown={(e) => e.preventDefault()}
		onclick={expanded ? handleSearch : expand}
		aria-label="Search"
	>
		<SearchIcon />
	</button>
</div>

<style>
	.search-container {
		position: relative;
		display: flex;
		align-items: center;
		width: 40px;
		height: 40px;
		transition: width 0.3s ease;
	}

	.search-container.expanded {
		width: 300px;
	}

	.search-input {
		position: absolute;
		right: 0;
		width: 100%;
		height: 100%;
		padding: var(--space-sm) 48px var(--space-sm) var(--space-md);
		background-color: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: 50%;
		color: var(--color-text);
		font-size: 0.875rem;
		opacity: 0;
		pointer-events: none;
		transition:
			opacity 0.3s ease,
	}

	.search-container.expanded .search-input {
		opacity: 1;
		pointer-events: auto;
		border-radius: var(--radius-full);
	}

	.search-input::placeholder {
		color: var(--color-text-muted);
	}

	.search-input:focus {
		outline: none;
		border-color: var(--color-primary);
	}

	.search-btn {
		position: absolute;
		right: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		padding: var(--space-sm);
		background-color: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: 50%;
		color: var(--color-text-muted);
		cursor: pointer;
		transition:
			background-color var(--transition-fast),
			color var(--transition-fast),
			border-radius 0.3s ease,
			border-color 0.3s ease;
	}

	.search-btn:active {
		background-color: var(--color-border);
	}

	.search-container.expanded .search-btn {
		background-color: transparent;
		border-color: transparent;
	}

	.search-container.expanded .search-btn:active {
		background-color: var(--color-surface-hover, rgba(255, 255, 255, 0.1));
	}

	@media (max-width: 640px) {
		.search-container.expanded {
			width: calc(100vw - var(--space-md) * 2 - 60px);
			max-width: 300px;
		}
	}
</style>
