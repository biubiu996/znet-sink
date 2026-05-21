<script lang="ts">
  import { getCoreConfigSnapshot } from '$lib/services/core';
  import type { CoreConfigSnapshot } from '$lib/types/core';

  let snapshot = $state<CoreConfigSnapshot | null>(null);

  async function refresh() {
    try {
      snapshot = await getCoreConfigSnapshot();
    } catch (e) {
      console.error('Failed to get core config:', e);
    }
  }

  $effect(() => {
    refresh();
  });
</script>

<div class="ccp-root desk-card">
  <div class="ccp-header">
    <span class="ccp-title">内核配置</span>
    <button class="ccp-refresh" onclick={refresh} title="刷新">
      <svg width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round">
        <path d="M10 6A4 4 0 1 1 6 2M6 2L9 2L9 5"/>
      </svg>
    </button>
  </div>

  {#if !snapshot}
    <div class="ccp-loading">加载中...</div>
  {:else}
    <div class="ccp-rows">
      <div class="config-row">
        <span class="config-label">内核</span>
        <span class="config-value">{snapshot.kernel}</span>
      </div>

      <div class="config-row">
        <span class="config-label">可执行文件</span>
        <span class="config-value mono">{snapshot.executablePath ?? '—'}</span>
      </div>

      <div class="config-row">
        <span class="config-label">文件存在</span>
        <span class="config-value {snapshot.executableExists ? 'status-ok' : 'status-warn'}">
          {snapshot.executableExists ? '是' : '否'}
        </span>
      </div>

      <div class="config-row">
        <span class="config-label">配置文件</span>
        <span class="config-value mono">{snapshot.configPath ?? '—'}</span>
      </div>

      <div class="config-row">
        <span class="config-label">工作目录</span>
        <span class="config-value mono">{snapshot.workingDir ?? '—'}</span>
      </div>

      <div class="config-row">
        <span class="config-label">Socket</span>
        <span class="config-value mono">{snapshot.endpoint.path}</span>
      </div>

      <div class="config-row">
        <span class="config-label">启动参数</span>
        <span class="config-value mono">{snapshot.launchArgs.join(' ') || '—'}</span>
      </div>
    </div>

    {#if snapshot.warnings.length > 0}
      <div class="ccp-warnings">
        <div class="warn-header">
          <svg width="11" height="11" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round">
            <line x1="6" y1="2.5" x2="6" y2="7"/>
            <circle cx="6" cy="9.5" r="0.6" fill="currentColor" stroke="none"/>
          </svg>
          警告
        </div>
        {#each snapshot.warnings as warning}
          <div class="warn-line">{warning}</div>
        {/each}
      </div>
    {/if}
  {/if}
</div>

<style>
  .ccp-root {
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .ccp-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 12px 8px;
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
  }

  .ccp-title {
    font-size: 12px;
    font-weight: 600;
    color: var(--foreground);
    letter-spacing: -0.01em;
  }

  .ccp-refresh {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 22px;
    height: 22px;
    border-radius: 5px;
    background: transparent;
    color: var(--muted-foreground);
    border: none;
    cursor: pointer;
    transition: background 0.12s ease, color 0.12s ease;
  }

  .ccp-refresh:hover {
    background: var(--muted);
    color: var(--foreground);
  }

  .ccp-loading {
    padding: 16px 12px;
    font-size: 12px;
    color: var(--muted-foreground);
  }

  .ccp-rows {
    padding: 6px 0;
  }

  /* Reusing the config-row pattern */
  .config-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 6px 12px;
    gap: 12px;
    transition: background 0.1s ease;
  }

  .config-row:hover {
    background: var(--muted);
  }

  .config-label {
    font-size: 12px;
    color: var(--muted-foreground);
    flex-shrink: 0;
    min-width: 72px;
  }

  .config-value {
    font-size: 12px;
    color: var(--foreground);
    text-align: right;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 200px;
  }

  .config-value.mono {
    font-family: var(--font-mono);
    font-size: 12px;
    color: var(--muted-foreground);
  }

  .config-value.status-ok {
    color: var(--success);
    font-weight: 500;
  }

  .config-value.status-warn {
    color: var(--warning);
    font-weight: 500;
  }

  /* Warnings block */
  .ccp-warnings {
    margin: 0 8px 8px;
    padding: 8px 10px;
    background: rgba(245, 158, 11, 0.08);
    border: 1px solid rgba(245, 158, 11, 0.2);
    border-radius: 7px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .warn-header {
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 12px;
    font-weight: 600;
    color: var(--warning);
  }

  .warn-line {
    font-size: 12px;
    color: var(--warning);
    opacity: 0.8;
    line-height: 1.4;
  }
</style>
