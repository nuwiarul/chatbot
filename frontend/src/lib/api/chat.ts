import { apiFetch } from '$lib/api/client';
import type { ChatRequest, ChatResponse } from '$lib/utils/types';
import { env } from '$lib/utils/env';
import type { ChatStreamEvent } from '$lib/utils/types';

export async function chat(req: ChatRequest): Promise<ChatResponse> {
  return apiFetch<ChatResponse>('/v1/chat', {
    method: 'POST',
    body: JSON.stringify(req)
  });
}

export async function chatStream(
  req: ChatRequest,
  onEvent: (event: ChatStreamEvent) => void
): Promise<void> {
  const { backendUrl, apiKey } = env();
  const res = await fetch(`${backendUrl}/v1/chat/stream`, {
    method: 'POST',
    headers: {
      'content-type': 'application/json',
      authorization: `Bearer ${apiKey}`
    },
    body: JSON.stringify(req)
  });

  if (!res.ok || !res.body) {
    throw new Error(`stream_failed status=${res.status}`);
  }

  const reader = res.body.getReader();
  const decoder = new TextDecoder();
  let buffer = '';

  while (true) {
    const { done, value } = await reader.read();
    if (done) break;
    buffer += decoder.decode(value, { stream: true });

    while (true) {
      const frameEnd = buffer.indexOf('\n\n');
      if (frameEnd === -1) break;

      const frame = buffer.slice(0, frameEnd);
      buffer = buffer.slice(frameEnd + 2);

      let eventType: string | null = null;
      let data: string | null = null;

      for (const rawLine of frame.split('\n')) {
        const line = rawLine.replace(/\r$/, '');
        if (line.startsWith('event:')) eventType = line.slice('event:'.length).trim();
        if (line.startsWith('data:')) {
          // Preserve leading spaces in deltas; only remove the optional single space after `data:`.
          const rest = line.slice('data:'.length);
          data = rest.startsWith(' ') ? rest.slice(1) : rest;
        }
      }

      if (!eventType || data == null) continue;

      if (eventType === 'delta') onEvent({ type: 'delta', data });
      else if (eventType === 'done') onEvent({ type: 'done', data: '[DONE]' });
      else if (eventType === 'keep-alive') onEvent({ type: 'keep-alive', data });
    }
  }
}
