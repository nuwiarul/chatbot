import { apiFetch } from '$lib/api/client';
import type { ChatRequest, ChatResponse } from '$lib/utils/types';

export async function chat(req: ChatRequest): Promise<ChatResponse> {
  return apiFetch<ChatResponse>('/v1/chat', {
    method: 'POST',
    body: JSON.stringify(req)
  });
}

