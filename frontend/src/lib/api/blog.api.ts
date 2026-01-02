import { PUBLIC_API_URL } from '$env/static/public';
import { ok, err, type Result } from '$lib/utils';
import type { GetBlogPostsResponse } from '$proto/blog';
import { GetBlogPostsRequest_Sort } from '$proto/blog';

export { GetBlogPostsRequest_Sort as BlogPostSort };

export async function getBlogPosts(
	query: string | undefined,
	limit: number,
	offset: number,
	sort: GetBlogPostsRequest_Sort,
): Promise<Result<GetBlogPostsResponse>> {
	try {
		const params = new URLSearchParams();
		if (query) params.set('query', query);
		params.set('limit', String(limit));
		params.set('offset', String(offset));
		params.set('sort', String(sort));

		const response = await fetch(`${PUBLIC_API_URL}/blog?${params}`, {
			credentials: 'include',
		});

		if (!response.ok) {
			return err('Failed to fetch blog posts');
		}

		const data: GetBlogPostsResponse = await response.json();
		return ok(data);
	} catch {
		return err('Network error');
	}
}
