import { PUBLIC_API_URL } from '$env/static/public';
import { createGrpcWebTransport } from '@connectrpc/connect-web';

export const transport = createGrpcWebTransport({
	baseUrl: PUBLIC_API_URL,
});
