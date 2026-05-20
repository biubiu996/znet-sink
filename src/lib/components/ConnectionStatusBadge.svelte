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

  function getStatusColor(): string {
    switch (status) {
      case 'proxy-active': return 'bg-green-500';
      case 'core-only': return 'bg-yellow-500';
      case 'crashed': return 'bg-red-500';
      default: return 'bg-muted';
    }
  }

  function getStatusText(): string {
    switch (status) {
      case 'proxy-active': return '代理运行中';
      case 'core-only': return '内核已启动';
      case 'crashed': return '内核已崩溃';
      default: return '系统未激活';
    }
  }
</script>

<div class="flex items-center gap-2 px-3 py-1 bg-card border border-card-border rounded-lg">
  <div class="w-2 h-2 rounded-full {getStatusColor()} {status === 'crashed' ? 'animate-pulse' : ''}"></div>
  <span class="text-xs font-medium text-foreground">{getStatusText()}</span>
</div>
