import { env } from '$lib/utils/env';

export type ApiError = {
  status: number;
  body: unknown;
};

export async function apiFetch<TResponse>(
  path: string,
  init?: Omit<RequestInit, 'headers'>
): Promise<TResponse> {
  const { backendUrl, apiKey } = env();
  const res = await fetch(`${backendUrl}${path}`, {
    ...init,
    headers: {
      'content-type': 'application/json',
      authorization: `Bearer ${apiKey}`
    }
  });

  if (!res.ok) {
    const body = (await res.json().catch(() => null)) as unknown;
    const error: ApiError = { status: res.status, body };
    throw error;
  }

  return (await res.json()) as TResponse;
}
