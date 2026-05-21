<script lang="ts">
  import { store } from '$lib/services/store.svelte';
  import type { ProxyNode } from '$lib/types/protocol';
  import { selectPolicy, probePolicy } from '$lib/services/core';

  const { nodes, initialSelected = '' }: {
    nodes: ProxyNode[];
    initialSelected?: string;
  } = $props();

  let selected = $state('');

  $effect(() => {
    selected = initialSelected;
  });
  let switching = $state<string | null>(null);
  let lastError = $state<string | null>(null);

  async function handleSelect(node: ProxyNode) {
    if (switching) return;
    switching = node.id;
    lastError = null;

    try {
      await probePolicy(node.name);
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

  function getDelayClass(delay: number): string {
    if (delay < 100) return 'delay-good';
    if (delay < 200) return 'delay-warn';
    return 'delay-bad';
  }
</script>

<div class="ns-root desk-card h-full flex flex-col overflow-hidden">
  <!-- Header -->
  <div class="ns-header">
    <span class="ns-label">核心策略出口</span>
    <span class="ns-badge">Radio</span>
  </div>

  <!-- List -->
  {#if nodes.length === 0}
    <div class="flex-1 flex items-center justify-center text-xs text-muted-foreground">等待节点数据...</div>
  {:else}
    <div class="ns-list">
      {#each nodes as node}
        <button
          class="ns-row {selected === node.id ? 'active' : ''} {switching === node.id ? 'switching' : ''}"
          onclick={() => handleSelect(node)}
          disabled={switching !== null || !store.isActionOperable('policies.select')}
        >
          <!-- Indicator dot -->
          <span class="ns-dot {selected === node.id ? 'dot-active' : ''}"></span>

          <!-- Name -->
          <span class="ns-name truncate">{node.name}</span>

          <!-- Switching hint -->
          {#if switching === node.id}
            <span class="ns-spin">⟳</span>
          {/if}

          <!-- Delay -->
          <span class="ns-delay {getDelayClass(node.delay)} ml-auto flex-shrink-0">
            {node.delay}<span style="font-size:10px;opacity:0.55;">ms</span>
          </span>
        </button>
      {/each}
    </div>
  {/if}

  <!-- Error -->
  {#if lastError}
    <div class="ns-error" title={lastError}>{lastError}</div>
  {/if}
</div>

<style>
  .ns-root {
    display: flex;
    flex-direction: column;
  }

  .ns-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 9px 12px 7px;
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
  }

  .ns-label {
    font-size: 12px;
    font-weight: 500;
    color: var(--muted-foreground);
  }

  .ns-badge {
    font-size: 10px;
    font-weight: 600;
    letter-spacing: 0.04em;
    text-transform: uppercase;
    color: var(--muted-foreground);
    background: var(--muted);
    padding: 2px 7px;
    border-radius: 4px;
  }

  .ns-list {
    flex: 1;
    overflow-y: auto;
    padding: 5px;
    display: flex;
    flex-direction: column;
    gap: 1px;
    min-height: 0;
  }

  .ns-row {
    display: flex;
    align-items: center;
    gap: 7px;
    width: 100%;
    padding: 7px 10px;
    border-radius: 6px;
    border: 1px solid transparent;
    background: transparent;
    cursor: pointer;
    transition: background 0.12s ease, border-color 0.12s ease;
    text-align: left;
  }

  .ns-row:hover {
    background: var(--muted);
  }

  .ns-row.active {
    background: var(--accent);
    border-color: rgba(99, 102, 241, 0.2);
  }

  :global(.dark) .ns-row.active {
    background: rgba(99, 102, 241, 0.1);
    border-color: rgba(165, 180, 252, 0.18);
  }

  .ns-row.switching {
    opacity: 0.55;
    pointer-events: none;
  }

  .ns-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: var(--muted-foreground);
    opacity: 0.3;
    flex-shrink: 0;
    transition: background 0.12s ease, opacity 0.12s ease;
  }

  .ns-dot.dot-active {
    background: var(--accent-foreground);
    opacity: 1;
  }

  :global(.dark) .ns-dot.dot-active {
    background: #A5B4FC;
  }

  .ns-name {
    font-size: 12px;
    font-weight: 500;
    color: var(--muted-foreground);
    flex: 1;
    min-width: 0;
    transition: color 0.12s ease;
  }

  .ns-row:hover .ns-name,
  .ns-row.active .ns-name {
    color: var(--foreground);
  }

  .ns-spin {
    font-size: 12px;
    color: var(--muted-foreground);
    animation: spin 0.8s linear infinite;
    flex-shrink: 0;
  }

  .ns-delay {
    font-size: 12px;
    font-family: var(--font-mono);
    font-weight: 700;
    letter-spacing: -0.02em;
    line-height: 1;
  }

  .delay-good { color: var(--success); }
  .delay-warn { color: var(--warning); }
  .delay-bad  { color: var(--destructive); }

  .ns-error {
    margin: 4px;
    padding: 7px 10px;
    background: rgba(239, 68, 68, 0.08);
    border: 1px solid rgba(239, 68, 68, 0.16);
    border-radius: 6px;
    font-size: 11px;
    color: var(--destructive);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    flex-shrink: 0;
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to   { transform: rotate(360deg); }
  }
</style>
