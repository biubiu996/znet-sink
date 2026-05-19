<script lang="ts">
  import type { CapabilityItem } from '$lib/types/capability';

  export let capability: CapabilityItem;
</script>

<div class="bg-card border border-card-border rounded-xl p-3 flex flex-col gap-2">
  <div class="flex items-center justify-between">
    <span class="text-xs font-medium text-foreground">{capability.key}</span>
    <div class="w-2 h-2 rounded-full {capability.enabled ? 'bg-green-500' : 'bg-muted'}"></div>
  </div>
  {#if capability.reason}
    <p class="text-[10px] text-muted-foreground">{capability.reason}</p>
  {/if}
</div>
