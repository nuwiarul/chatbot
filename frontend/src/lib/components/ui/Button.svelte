<script lang="ts">
  export let type: 'button' | 'submit' = 'button';
  export let disabled = false;
</script>

<button
  class="rounded bg-black px-3 py-2 text-sm font-medium text-white disabled:opacity-50"
  {type}
  {disabled}
>
  <slot />
</button>

