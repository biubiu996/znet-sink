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

  function getDelayColor(delay: number): string {
    if (delay < 100) return 'var(--success)';
    if (delay < 200) return 'var(--warning)';
    return 'var(--destructive)';
  }

  function getDelayClass(delay: number): string {
    if (delay < 100) return 'delay-good';
    if (delay < 200) return 'delay-warn';
    return 'delay-bad';
  }
</script>

<div class="flex-1 w-full grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-2 overflow-y-auto content-start p-1">
  {#each nodes as node}
    <button
      onclick={() => handleSelect(node)}
      disabled={switching !== null}
      class="node-tile {store.selectedNodeId === node.id ? 'active' : ''} {switching === node.id ? 'switching' : ''}"
    >
      <!-- Top: name + check -->
      <div class="flex items-start justify-between gap-1 w-full">
        <span class="node-name truncate">{node.name}</span>
        {#if showCheck && store.selectedNodeId === node.id}
          <span class="node-check" aria-hidden="true">
            <svg width="10" height="10" viewBox="0 0 10 10" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="2,5 4,7 8,3"/>
            </svg>
          </span>
        {/if}
      </div>
      <!-- Bottom: domain + delay -->
      <div class="flex items-center justify-between gap-1 w-full mt-auto">
        <span class="node-domain truncate">{node.domain}</span>
        <span class="node-delay {getDelayClass(node.delay)}">
          {#if switching === node.id}
            <span class="node-switching">…</span>
          {:else}
            {node.delay}<span style="opacity:0.55;font-size:10px;">ms</span>
          {/if}
        </span>
      </div>
      <!-- Delay indicator bar (bottom) -->
      <div class="node-bar" style="background: {getDelayColor(node.delay)};"></div>
    </button>
  {/each}
</div>

<style>
  .node-tile {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 3px;
    padding: 9px 11px 11px;
    height: 70px;
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 8px;
    cursor: pointer;
    overflow: hidden;
    transition: background 0.13s ease, border-color 0.13s ease, box-shadow 0.13s ease;
    text-align: left;
  }

  .node-tile:hover {
    background: var(--surface);
    border-color: rgba(128, 128, 160, 0.18);
  }

  .node-tile.active {
    background: var(--accent);
    border-color: var(--accent-foreground);
    border-color: rgba(99, 102, 241, 0.24);
    box-shadow: 0 0 0 1px rgba(99,102,241,0.12);
  }

  :global(.dark) .node-tile.active {
    background: rgba(99, 102, 241, 0.1);
    border-color: rgba(165, 180, 252, 0.22);
    box-shadow: 0 0 0 1px rgba(165,180,252,0.1);
  }

  .node-tile.switching {
    opacity: 0.55;
    pointer-events: none;
  }

  .node-name {
    font-size: 12px;
    font-weight: 600;
    color: var(--foreground);
    line-height: 1.2;
    letter-spacing: -0.01em;
    max-width: calc(100% - 22px);
  }

  .node-tile.active .node-name {
    color: var(--accent-foreground);
  }

  :global(.dark) .node-tile.active .node-name {
    color: #A5B4FC;
  }

  .node-domain {
    font-size: 10px;
    font-family: var(--font-mono);
    color: var(--muted-foreground);
    opacity: 0.6;
    flex: 1;
    min-width: 0;
  }

  .node-delay {
    font-size: 12px;
    font-family: var(--font-mono);
    font-weight: 700;
    letter-spacing: -0.02em;
    flex-shrink: 0;
    line-height: 1;
  }

  .node-delay.delay-good { color: var(--success); }
  .node-delay.delay-warn { color: var(--warning); }
  .node-delay.delay-bad  { color: var(--destructive); }

  .node-switching {
    font-size: 11px;
    animation: pulse 1s infinite;
    color: var(--muted-foreground);
  }

  .node-check {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    background: rgba(99,102,241,0.18);
    color: var(--accent-foreground);
    flex-shrink: 0;
    margin-top: 1px;
  }

  :global(.dark) .node-check {
    background: rgba(165,180,252,0.18);
    color: #A5B4FC;
  }

  /* Bottom accent bar — 2px color strip */
  .node-bar {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    height: 2px;
    opacity: 0.35;
    border-radius: 0 0 8px 8px;
    transition: opacity 0.13s ease;
  }

  .node-tile:hover .node-bar,
  .node-tile.active .node-bar {
    opacity: 0.65;
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.3; }
  }
</style>
