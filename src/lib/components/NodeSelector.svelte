<script lang="ts">
  import type { ProxyNode } from '$lib/types/protocol';
  import { selectPolicy, probePolicy } from '$lib/services/core';

  const { nodes, initialSelected = '' }: {
    nodes: ProxyNode[];
    initialSelected?: string;
  } = $props();

  let selected = $state(initialSelected);
  let switching = $state<string | null>(null);
  let lastError = $state<string | null>(null);

  async function handleSelect(node: ProxyNode) {
    if (switching) return;
    switching = node.id;
    lastError = null;

    try {
      // Probe first to verify the target is reachable
      await probePolicy(node.name);
      // Then switch
      const result = await selectPolicy('proxy', node.name);
      if (result.error) {
        lastError = result.error.message;
      } else {
        selected = node.id;
      }
    } catch (e) {
      lastError = String(e);
    } finally {
      switching = null;
    }
  }
</script>

<div class="bg-card border border-card-border rounded-xl p-3 flex flex-col h-full">
  <div class="flex items-center justify-between mb-3 flex-shrink-0 overflow-hidden">
    <span class="text-sm font-medium text-muted-foreground truncate">核心策略出口</span>
    <span class="text-xs text-muted-foreground flex-shrink-0 ml-2">Radio</span>
  </div>

  {#if nodes.length === 0}
    <div class="flex-1 flex items-center justify-center text-[10px] text-muted-foreground">等待节点数据...</div>
  {:else}
    <div class="flex flex-col gap-1 flex-1 overflow-y-auto min-h-0">
      {#each nodes as node}
        <button
          onclick={() => handleSelect(node)}
          disabled={switching !== null}
          class="w-full px-3 py-2 rounded-lg text-left text-sm font-medium transition-all flex items-center justify-between flex-shrink-0 overflow-hidden
                 {selected === node.id
                   ? 'bg-muted text-foreground border border-card-border'
                   : 'text-muted-foreground hover:bg-muted/50'}
                 {switching === node.id ? 'opacity-60' : ''}"
        >
          <span class="truncate">
            {node.name}
            {#if switching === node.id}
              <span class="text-[10px] text-muted-foreground ml-1">切换中...</span>
            {/if}
          </span>
          {#if selected === node.id}
            <span class="text-xs text-muted-foreground flex-shrink-0 ml-2">{node.delay}ms</span>
          {/if}
        </button>
      {/each}
    </div>
  {/if}

  {#if lastError}
    <div class="mt-2 text-[10px] text-red-500 truncate flex-shrink-0" title={lastError}>
      {lastError}
    </div>
  {/if}
</div>
