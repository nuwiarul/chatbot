<script lang="ts">
  import type { ChatMessage } from '$lib/utils/types';
  import Button from '$lib/components/ui/Button.svelte';
  import { chatStream } from '$lib/api/chat';

  let input = '';
  let messages: ChatMessage[] = [{ role: 'assistant', content: 'Halo! Tanyakan apa saja.' }];
  let sending = false;

  async function send(): Promise<void> {
    const text = input.trim();
    if (text.length === 0) return;
    if (sending) return;

    const baseMessages: ChatMessage[] = [...messages, { role: 'user', content: text }];
    messages = baseMessages;
    input = '';

    try {
      sending = true;
      // Add an empty assistant message that will be filled by streaming deltas
      let assistantIndex = -1;
      messages = [
        ...baseMessages,
        { role: 'assistant', content: '' }
      ];
      assistantIndex = messages.length - 1;

      await chatStream({ messages: baseMessages }, (ev) => {
        if (ev.type === 'delta') {
          const current = messages[assistantIndex];
          messages = [
            ...messages.slice(0, assistantIndex),
            { ...current, content: current.content + ev.data },
            ...messages.slice(assistantIndex + 1)
          ];
        }
      });
    } finally {
      sending = false;
    }
  }
</script>

<div class="flex h-[calc(100vh-2rem)] flex-col gap-3 rounded border p-4">
  <div class="flex-1 space-y-2 overflow-auto">
    {#each messages as m}
      <div class="text-sm">
        <span class="font-semibold">{m.role}:</span>
        <span class="ml-2">{m.content}</span>
      </div>
    {/each}
  </div>

  <form
    class="flex gap-2"
    on:submit|preventDefault={() => {
      void send();
    }}
  >
    <input
      class="w-full rounded border px-3 py-2 text-sm"
      placeholder="Tulis pesan…"
      disabled={sending}
      bind:value={input}
    />
    <Button type="submit" disabled={sending}>Send</Button>
  </form>
</div>
