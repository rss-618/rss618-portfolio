import { createClient } from '@connectrpc/connect';
import { BlogService, GetBlogPostsRequest_Sort } from '$proto/blog_pb';
import type { GetBlogPostsResponse } from '$proto/blog_pb';
import { ok, err, type Result } from '$lib/utils';
import { transport } from './transport';

const client = createClient(BlogService, transport);

export async function getBlogPosts(
	query: string | undefined,
	limit: number,
	offset: number,
	sort: GetBlogPostsRequest_Sort
): Promise<Result<GetBlogPostsResponse>> {
	try {
		const response = await client.getBlogPosts({ query, limit, offset, sort });
		return ok(response);
	} catch (e) {
		return err(e instanceof Error ? e.message : 'Network error');
	}
}
