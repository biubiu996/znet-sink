<script lang="ts">
  import { getCoreProcessStatus, getSystemProxyStatus } from '$lib/services/core';
  import type { CoreProcessStatus } from '$lib/types/core';
  import { store } from '$lib/services/store.svelte';

  type ProxyStatus = { enabled: boolean };
  let coreStatus = $state<CoreProcessStatus | null>(null);
  let proxyStatus = $state<ProxyStatus | null>(null);

  async function refresh() {
    try {
      [coreStatus, proxyStatus] = await Promise.all([
        getCoreProcessStatus(),
        getSystemProxyStatus(),
      ]);
    } catch (e) {
      console.error('Failed to refresh status:', e);
    }
  }

  $effect(() => {
    if (store.isInitialized) {
      refresh();
      const interval = setInterval(refresh, 3000);
      return () => clearInterval(interval);
    }
  });

  const isCoreRunning = $derived(coreStatus?.state === 'running');
  const isCoreStarting = $derived(coreStatus?.state === 'starting');
  const isCoreCrashed = $derived(coreStatus?.exitReason === 'crashed');
  const isProxyEnabled = $derived(proxyStatus?.enabled === true);

  type Status = 'off' | 'core-only' | 'proxy-active' | 'crashed';
  const status: Status = $derived(
    isCoreCrashed ? 'crashed' :
    isProxyEnabled && isCoreRunning ? 'proxy-active' :
    isCoreRunning || isCoreStarting ? 'core-only' :
    'off'
  );

  const dotColor = $derived(
    status === 'proxy-active' ? '#22C55E' :
    status === 'core-only'   ? '#F59E0B' :
    status === 'crashed'     ? '#EF4444' :
    'var(--muted-foreground)'
  );

  const label = $derived(
    status === 'proxy-active' ? '运行中' :
    status === 'core-only'   ? '内核运行' :
    status === 'crashed'     ? '异常' :
    '未激活'
  );
</script>

<div class="status-badge" class:crashed={status === 'crashed'} class:active={status === 'proxy-active'}>
  <span
    class="status-dot"
    class:pulse={status === 'crashed' || status === 'core-only'}
    style="background: {dotColor};"
  ></span>
  <span class="status-label">{label}</span>
</div>

<style>
  .status-badge {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 4px 9px;
    border-radius: 6px;
    border: 1px solid var(--border);
    background: var(--surface, var(--card));
  }

  .status-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .status-dot.pulse {
    animation: pulse-dot 1.8s ease-in-out infinite;
  }

  .status-label {
    font-size: 11.5px;
    font-weight: 500;
    color: var(--muted-foreground);
    letter-spacing: 0.01em;
    white-space: nowrap;
  }

  .status-badge.crashed .status-label {
    color: var(--destructive);
  }

  .status-badge.active .status-label {
    color: var(--success);
  }

  @keyframes pulse-dot {
    0%, 100% { opacity: 1; }
    50%       { opacity: 0.35; }
  }
</style>
