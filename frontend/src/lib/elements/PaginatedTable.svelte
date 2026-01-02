<script lang="ts" generics="RowData, SortValue extends number">
	import type { Snippet } from 'svelte';
	import SearchButton from '$lib/elements/SearchButton.svelte';
	import FilterButton from '$lib/elements/FilterButton.svelte';
	import ChevronLeftIcon from '$lib/assets/icons/ChevronLeftIcon.svelte';
	import ChevronRightIcon from '$lib/assets/icons/ChevronRightIcon.svelte';

	interface SortOption {
		value: SortValue;
		label: string;
	}

	interface Props {
		rows: RowData[];
		total: number;
		limit: number;
		offset: number;
		sort: SortValue;
		sortOptions: SortOption[];
		loading?: boolean;
		searchPlaceholder?: string;
		onSearch: (query: string) => void;
		onPageChange: (offset: number) => void;
		onSortChange: (sort: SortValue) => void;
		header: Snippet;
		row: Snippet<[RowData]>;
	}

	let {
		rows,
		total,
		limit,
		offset,
		sort,
		sortOptions,
		loading = false,
		searchPlaceholder = 'Search...',
		onSearch,
		onPageChange,
		onSortChange,
		header,
		row,
	}: Props = $props();

	const currentPage = $derived(Math.floor(offset / limit) + 1);
	const totalPages = $derived(Math.ceil(total / limit));
	const startItem = $derived(total === 0 ? 0 : offset + 1);
	const endItem = $derived(Math.min(offset + limit, total));
	const hasResults = $derived(total > 0);
	const showPagination = $derived(totalPages > 1);

	function goToPage(page: number) {
		if (page < 1 || page > totalPages) return;
		onPageChange((page - 1) * limit);
	}
</script>

<div class="paginated-table">
	<div class="controls">
		<SearchButton placeholder={searchPlaceholder} {onSearch} />
		<FilterButton value={sort} options={sortOptions} onChange={onSortChange} />
	</div>

	{#if hasResults}
		<div class="table-container" class:loading>
			<table>
				<thead>
					{@render header()}
				</thead>
				<tbody>
					{#each rows as item}
						{@render row(item)}
					{/each}
				</tbody>
			</table>
		</div>

		{#if showPagination}
			<div class="pagination">
				<span class="page-info">
					{startItem}-{endItem} of {total}
				</span>

				<div class="page-controls">
					<button
						class="page-btn"
						onclick={() => goToPage(currentPage - 1)}
						disabled={currentPage === 1}
						aria-label="Previous page"
					>
						<ChevronLeftIcon />
					</button>
					<button
						class="page-btn"
						onclick={() => goToPage(currentPage + 1)}
						disabled={currentPage >= totalPages}
						aria-label="Next page"
					>
						<ChevronRightIcon />
					</button>
				</div>
			</div>
		{/if}
	{:else if !loading}
		<div class="no-results">
			<p>No results found</p>
		</div>
	{/if}
</div>

<style>
	.paginated-table {
		display: flex;
		flex-direction: column;
		gap: var(--space-md);
	}

	.controls {
		display: flex;
		gap: var(--space-md);
		justify-content: flex-end;
	}

	.table-container {
		overflow-x: auto;
		transition: opacity var(--transition-fast);
	}

	.table-container.loading {
		opacity: 0.5;
		pointer-events: none;
	}

	table {
		width: 100%;
		border-collapse: collapse;
	}

	:global(.paginated-table th),
	:global(.paginated-table td) {
		padding: var(--space-sm) var(--space-md);
		text-align: left;
		border-bottom: 1px solid var(--color-border);
	}

	:global(.paginated-table th) {
		font-weight: 600;
		color: var(--color-text-muted);
		font-size: 0.75rem;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	:global(.paginated-table td) {
		color: var(--color-text);
		font-size: 0.875rem;
	}

	:global(.paginated-table tbody tr:hover) {
		background-color: var(--color-surface);
	}

	.no-results {
		text-align: center;
		padding: var(--space-xl);
		color: var(--color-text-muted);
	}

	.no-results p {
		margin: 0;
	}

	.pagination {
		display: flex;
		align-items: center;
		justify-content: flex-end;
		gap: var(--space-lg);
	}

	.page-info {
		color: var(--color-text-muted);
		font-size: 0.875rem;
	}

	.page-controls {
		display: flex;
		gap: var(--space-xs);
	}

	.page-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		padding: 0;
		background-color: transparent;
		border: none;
		border-radius: 50%;
		color: var(--color-text-muted);
		cursor: pointer;
		transition:
			background-color var(--transition-fast),
			color var(--transition-fast);
	}

	.page-btn:hover:not(:disabled) {
		background-color: var(--color-surface);
		color: var(--color-text);
	}

	.page-btn:disabled {
		opacity: 0.3;
		cursor: not-allowed;
	}
</style>
