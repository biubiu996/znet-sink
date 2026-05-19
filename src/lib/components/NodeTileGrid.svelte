<script lang="ts">
  import { store } from '$lib/services/store.svelte';
  import { selectPolicy, probePolicy } from '$lib/services/core';
  import type { ProxyNode } from '$lib/types/protocol';

  interface Props {
    nodes: ProxyNode[];
    showCheck?: boolean;
  }
  let { nodes, showCheck = false }: Props = $props();

  let switching = $state<string | null>(null);

  async function handleSelect(node: ProxyNode) {
    if (switching) return;
    switching = node.id;
    try {
      await probePolicy(node.name);
      const result = await selectPolicy('proxy', node.name);
      if (!result.error) {
        store.selectedNodeId = node.id;
      }
    } catch (e) {
      console.error('Policy switch failed:', e);
    } finally {
      switching = null;
    }
  }
</script>

<div class="flex-1 w-full grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-3 overflow-y-auto content-start">
  {#each nodes as node}
    <button
      onclick={() => handleSelect(node)}
      disabled={switching !== null}
      class="p-4 rounded-2xl border text-left transition-all duration-150 flex flex-col justify-between h-24 bg-card
             {store.selectedNodeId === node.id ? 'border-primary shadow-md bg-foreground/[0.02] ring-1 ring-border' : 'border-card-border hover:border-muted-foreground/30'}
             {switching === node.id ? 'opacity-60' : ''}"
    >
      <div class="w-full flex justify-between items-start">
        <div class="flex flex-col gap-0.5">
          <span class="text-xs font-bold {store.selectedNodeId === node.id ? 'text-primary' : 'text-foreground'}">{node.name}</span>
          <span class="text-[10px] text-muted-foreground font-mono">{node.domain}</span>
        </div>
        {#if showCheck && store.selectedNodeId === node.id}
          <span class="text-primary font-black text-sm">✓</span>
        {/if}
      </div>
      <div class="w-full flex justify-end items-center gap-2 text-xs font-mono font-bold {store.selectedNodeId === node.id ? 'text-primary' : 'text-muted-foreground/70'}">
        {#if switching === node.id}
          <span class="text-[10px]">切换中...</span>
        {:else}
          {node.delay} ms
        {/if}
      </div>
    </button>
  {/each}
</div>
