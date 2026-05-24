import { apiFetch } from '$lib/api/client';

export type HealthResponse = {
  ok: boolean;
  db: boolean;
};

export async function getHealth(): Promise<HealthResponse> {
  return apiFetch<HealthResponse>('/health', { method: 'GET' });
}

