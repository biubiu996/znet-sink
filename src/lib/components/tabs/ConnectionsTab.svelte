<script lang="ts">
  import { queryFlows, closeFlow, type FlowInfo } from '$lib/services/core';
  import { store } from '$lib/services/store.svelte';
  import { overviewData } from '$lib/services/overview-data.svelte';

  let flows = $state<FlowInfo[]>([]);
  let loading = $state(true);
  let closingId = $state<string | null>(null);

  async function refresh() {
    loading = true;
    try {
      flows = await queryFlows();
    } catch (e) {
      console.error('Failed to query flows:', e);
    } finally {
      loading = false;
    }
  }

  async function handleClose(flowId: string) {
    closingId = flowId;
    try {
      await closeFlow(flowId);
      flows = flows.filter(f => f.flowId !== flowId);
    } catch (e) {
      console.error('Failed to close flow:', e);
    } finally {
      closingId = null;
    }
  }

  function formatBytes(bytes: number): string {
    if (bytes >= 1_000_000_000) return `${(bytes / 1_000_000_000).toFixed(2)} GB`;
    if (bytes >= 1_000_000) return `${(bytes / 1_000_000).toFixed(1)} MB`;
    if (bytes >= 1_000) return `${(bytes / 1_000).toFixed(0)} KB`;
    return `${bytes} B`;
  }

  function formatDuration(startedAtMs: number): string {
    const elapsed = Date.now() - startedAtMs;
    if (elapsed < 0) return '0s';
    const sec = Math.floor(elapsed / 1000);
    if (sec < 60) return `${sec}s`;
    const min = Math.floor(sec / 60);
    if (min < 60) return `${min}m ${sec % 60}s`;
    const hr = Math.floor(min / 60);
    return `${hr}h ${min % 60}m`;
  }

  $effect(() => {
    refresh();
    const interval = setInterval(refresh, 3000);
    return () => clearInterval(interval);
  });
</script>

<div class="desk-card flex-1 overflow-hidden flex flex-col animate-fade-in">
  <!-- Panel header -->
  <div class="panel-header">
    <div class="panel-title-row">
      <span class="panel-title">活跃连接</span>
      <span class="count-badge">{overviewData.activeConnections} 个</span>
    </div>
    <button class="action-btn" onclick={refresh}>
      <svg width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round">
        <path d="M10 6A4 4 0 1 1 6 2M6 2L9 2L9 5"/>
      </svg>
      刷新
    </button>
  </div>

  <!-- Content -->
  {#if loading && flows.length === 0}
    <div class="panel-empty">加载中...</div>
  {:else if flows.length === 0}
    <div class="panel-empty-block">
      <span class="empty-title">无活跃连接</span>
      <span class="empty-desc">内核未运行或暂无流量</span>
    </div>
  {:else}
    <div class="list-scroll">
      {#each flows as flow (flow.flowId)}
        <div class="flow-row">
          <div class="flow-main">
            <!-- Top line: ID + protocol -->
            <div class="flow-top">
              <span class="flow-id">{flow.flowId}</span>
              <span class="row-tag flow-protocol">{flow.protocol}</span>
            </div>
            <!-- Middle line: source → destination -->
            <div class="flow-route">
              <span class="flow-src">{flow.source}</span>
              <span class="flow-arrow">→</span>
              <span class="flow-dst">{flow.destination}</span>
            </div>
            <!-- Bottom line: stats -->
            <div class="flow-stats">
              <span class="flow-stat up">↑ {formatBytes(flow.bytesUp)}</span>
              <span class="flow-stat down">↓ {formatBytes(flow.bytesDown)}</span>
              <span class="flow-dur">{formatDuration(flow.startedAtUnixMs)}</span>
            </div>
          </div>

          {#if store.isActionOperable('core.flow.close')}
            <button
              class="flow-close"
              onclick={() => handleClose(flow.flowId)}
              disabled={closingId === flow.flowId}
              title="关闭连接"
            >
              <svg width="14" height="14" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round">
                <line x1="2" y1="2" x2="10" y2="10"/><line x1="10" y1="2" x2="2" y2="10"/>
              </svg>
            </button>
          {/if}
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  /* Panel */
  .panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 11px 14px 10px;
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
  }

  .panel-title-row {
    display: flex;
    align-items: center;
    gap: 7px;
  }

  .panel-title {
    font-size: 13px;
    font-weight: 600;
    color: var(--foreground);
    letter-spacing: -0.01em;
  }

  .count-badge {
    font-size: 12px;
    font-weight: 600;
    font-family: var(--font-mono);
    padding: 2px 8px;
    border-radius: 5px;
    background: var(--muted);
    color: var(--muted-foreground);
  }

  .action-btn {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    padding: 5px 10px;
    border-radius: 7px;
    font-size: 12px;
    font-weight: 500;
    background: var(--muted);
    color: var(--foreground);
    border: 1px solid var(--border);
    cursor: pointer;
    transition: background 0.12s ease;
  }

  .action-btn:hover { background: var(--surface); }

  .panel-empty {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 12px;
    color: var(--muted-foreground);
  }

  .panel-empty-block {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 5px;
    padding: 28px;
  }

  .empty-title {
    font-size: 12px;
    color: var(--muted-foreground);
  }

  .empty-desc {
    font-size: 12px;
    color: var(--muted-foreground);
    opacity: 0.6;
  }

  /* List */
  .list-scroll {
    flex: 1;
    overflow-y: auto;
    padding: 5px;
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-height: 0;
  }

  /* Flow row */
  .flow-row {
    display: flex;
    align-items: flex-start;
    gap: 8px;
    padding: 10px 11px;
    border-radius: 8px;
    border: 1px solid transparent;
    transition: background 0.12s ease, border-color 0.12s ease;
  }

  .flow-row:hover {
    background: var(--muted);
    border-color: var(--border);
  }

  .flow-main {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 3px;
  }

  .flow-top {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .flow-id {
    font-size: 12px;
    font-weight: 600;
    color: var(--foreground);
    font-family: var(--font-mono);
  }

  .row-tag {
    font-size: 12px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    padding: 2px 6px;
    border-radius: 4px;
    background: var(--muted);
    color: var(--muted-foreground);
  }

  .flow-protocol {
    padding: 1px 5px;
    font-size: 12px;
    letter-spacing: 0.06em;
  }

  .flow-route {
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 12px;
    color: var(--muted-foreground);
    overflow: hidden;
  }

  .flow-src, .flow-dst {
    font-family: var(--font-mono);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .flow-src { max-width: 140px; }
  .flow-dst { max-width: 160px; }

  .flow-arrow {
    flex-shrink: 0;
    font-size: 12px;
    opacity: 0.4;
    padding: 0 1px;
  }

  .flow-stats {
    display: flex;
    align-items: center;
    gap: 10px;
    font-size: 12px;
  }

  .flow-stat {
    font-weight: 500;
    font-family: var(--font-mono);
  }

  .flow-stat.up { color: rgba(34, 197, 94, 0.85); }
  .flow-stat.down { color: rgba(59, 130, 246, 0.85); }

  .flow-dur {
    color: var(--muted-foreground);
    opacity: 0.6;
    font-family: var(--font-mono);
  }

  .flow-close {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    border-radius: 6px;
    background: transparent;
    color: var(--muted-foreground);
    border: none;
    cursor: pointer;
    opacity: 0;
    transition: opacity 0.12s ease, background 0.12s ease, color 0.12s ease;
    flex-shrink: 0;
    margin-top: 2px;
  }

  .flow-row:hover .flow-close { opacity: 1; }

  .flow-close:hover {
    background: rgba(239, 68, 68, 0.1);
    color: var(--destructive);
  }

  .flow-close:disabled { opacity: 0.3; cursor: not-allowed; }
</style>
