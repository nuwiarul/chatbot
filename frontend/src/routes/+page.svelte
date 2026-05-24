<script lang="ts">
  import ChatPane from '$lib/components/chat/ChatPane.svelte';
  import { onMount } from 'svelte';
  import { getHealth } from '$lib/api/health';

  let healthText = 'checking...';

  onMount(async () => {
    try {
      const h = await getHealth();
      healthText = `ok=${h.ok} db=${h.db}`;
    } catch (e) {
      healthText = 'backend unreachable';
    }
  });
</script>

<main class="mx-auto max-w-3xl p-4">
  <div class="mb-3 text-xs text-gray-600">health: {healthText}</div>
  <ChatPane />
</main>

