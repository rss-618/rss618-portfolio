<script lang="ts">
	import PaginatedTable from '$lib/elements/PaginatedTable.svelte';
	import { getBlogPosts } from '$lib/api/blog.api';
	import { GetBlogPostsRequest_Sort, type BlogPostSummary } from '$proto/blog_pb';

	const SORT_OPTIONS = [
		{ value: GetBlogPostsRequest_Sort.RELEVANCE, label: 'Relevance' },
		{ value: GetBlogPostsRequest_Sort.CREATED_DESC, label: 'Newest' },
		{ value: GetBlogPostsRequest_Sort.CREATED_ASC, label: 'Oldest' },
		{ value: GetBlogPostsRequest_Sort.UPDATED_DESC, label: 'Recently Updated' },
		{ value: GetBlogPostsRequest_Sort.UPDATED_ASC, label: 'Least Recently Updated' },
	];

	const LIMIT = 10;

	let posts = $state<BlogPostSummary[]>([]);
	let total = $state(0);
	let offset = $state(0);
	let sort = $state(GetBlogPostsRequest_Sort.CREATED_DESC);
	let query = $state<string | undefined>(undefined);
	let loading = $state(false);

	async function fetchPosts() {
		loading = true;
		const result = await getBlogPosts(query, LIMIT, offset, sort);
		if (result.success) {
			posts = result.data.posts;
			total = result.data.total;
		}
		loading = false;
	}

	function handleSearch(searchQuery: string) {
		query = searchQuery || undefined;
		offset = 0;
		fetchPosts();
	}

	function handlePageChange(newOffset: number) {
		offset = newOffset;
		fetchPosts();
	}

	function handleSortChange(newSort: GetBlogPostsRequest_Sort) {
		sort = newSort;
		offset = 0;
		fetchPosts();
	}

	function formatDate(timestamp: bigint): string {
		return new Date(Number(timestamp) * 1000).toLocaleDateString();
	}

	$effect(() => {
		fetchPosts();
	});
</script>

<PaginatedTable
	rows={posts}
	{total}
	limit={LIMIT}
	{offset}
	{sort}
	sortOptions={SORT_OPTIONS}
	{loading}
	searchPlaceholder="Search blog posts..."
	onSearch={handleSearch}
	onPageChange={handlePageChange}
	onSortChange={handleSortChange}
>
	{#snippet header()}
		<tr>
			<th>Title</th>
			<th>Description</th>
			<th>Created</th>
			<th>Updated</th>
		</tr>
	{/snippet}

	{#snippet row(post: BlogPostSummary)}
		<tr>
			<td>{post.title}</td>
			<td class="description">{post.description}</td>
			<td>{formatDate(post.createdAt)}</td>
			<td>{formatDate(post.updatedAt)}</td>
		</tr>
	{/snippet}
</PaginatedTable>

<style>
	.description {
		max-width: 300px;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}
</style>
