<script lang="ts">
  import { store } from '$lib/services/store.svelte';
  import { overviewData } from '$lib/services/overview-data.svelte';
  import { enableSystemProxy, disableSystemProxy, getSystemProxyStatus } from '$lib/services/core';
  import NodeTileGrid from '$lib/components/NodeTileGrid.svelte';
  import TrafficChart from '$lib/components/TrafficChart.svelte';
  import NodeSelector from '$lib/components/NodeSelector.svelte';
  import CoreStatusCard from '$lib/components/core/CoreStatusCard.svelte';
  import LogPanel from '$lib/components/core/LogPanel.svelte';

  let systemProxyEnabled = $state(false);
  let tunEnabled = $state(false);

  async function checkProxyStatus() {
    try {
      const status = await getSystemProxyStatus();
      systemProxyEnabled = status.enabled;
    } catch {
      // Backend may not be available
    }
  }

  async function toggleSystemProxy() {
    try {
      if (systemProxyEnabled) {
        await disableSystemProxy();
        systemProxyEnabled = false;
      } else {
        await enableSystemProxy();
        systemProxyEnabled = true;
      }
    } catch (e) {
      console.error('Failed to toggle system proxy:', e);
    }
  }

  $effect(() => {
    checkProxyStatus();
  });
</script>

{#if store.uiMode === 'pro'}
  <div class="flex-1 w-full flex flex-col gap-3 overflow-y-auto overflow-x-hidden animate-fade-in min-h-0">
    <!-- 顶部状态卡片 -->
    <div class="grid grid-cols-3 gap-3 flex-shrink-0">
      <CoreStatusCard />

      <!-- 系统代理开关 -->
      <div class="bg-card border border-card-border rounded-xl p-3 flex flex-col justify-between h-24 overflow-hidden">
        <span class="text-sm font-medium text-muted-foreground truncate">系统代理</span>
        <button
          onclick={toggleSystemProxy}
          class="w-10 h-6 rounded-full relative transition-colors flex-shrink-0 {systemProxyEnabled ? 'bg-primary' : 'bg-muted'}"
          aria-label={systemProxyEnabled ? '关闭系统代理' : '开启系统代理'}
        >
          <div class="w-5 h-5 rounded-full bg-white absolute top-0.5 transition-all shadow {systemProxyEnabled ? 'left-4.5' : 'left-0.5'}"></div>
        </button>
      </div>

      <!-- TUN 模式开关 -->
      <div class="bg-card border border-card-border rounded-xl p-3 flex flex-col justify-between h-24 overflow-hidden">
        <span class="text-sm font-medium text-muted-foreground truncate">TUN 虚拟网卡</span>
        <button
          onclick={() => tunEnabled = !tunEnabled}
          class="w-10 h-6 rounded-full relative transition-colors flex-shrink-0 {tunEnabled ? 'bg-primary' : 'bg-muted'}"
          aria-label={tunEnabled ? '关闭TUN模式' : '开启TUN模式'}
        >
          <div class="w-5 h-5 rounded-full bg-white absolute top-0.5 transition-all shadow {tunEnabled ? 'left-4.5' : 'left-0.5'}"></div>
        </button>
      </div>
    </div>

    <!-- 中间区域：流量看板 + 节点选择 -->
    <div class="flex-1 w-full flex gap-3 overflow-hidden min-h-[200px]">
      <div class="w-2/3 overflow-hidden">
        <TrafficChart history={overviewData.speedHistory} />
      </div>
      <NodeSelector nodes={overviewData.proxyNodes} />
    </div>

    <!-- 底部日志 -->
    <div class="h-40 flex-shrink-0">
      <LogPanel />
    </div>
  </div>
{:else}
  <div class="flex-1 w-full flex flex-col gap-3 overflow-y-auto overflow-x-hidden animate-fade-in min-h-0">
    <div class="grid grid-cols-3 gap-3 flex-shrink-0">
      <CoreStatusCard />

      <!-- 系统代理开关 -->
      <div class="bg-card border border-card-border rounded-xl p-3 flex flex-col justify-between h-24 overflow-hidden">
        <span class="text-sm font-medium text-muted-foreground truncate">系统代理</span>
        <button
          onclick={toggleSystemProxy}
          class="w-10 h-6 rounded-full relative transition-colors flex-shrink-0 {systemProxyEnabled ? 'bg-primary' : 'bg-muted'}"
          aria-label={systemProxyEnabled ? '关闭系统代理' : '开启系统代理'}
        >
          <div class="w-5 h-5 rounded-full bg-white absolute top-0.5 transition-all shadow {systemProxyEnabled ? 'left-4.5' : 'left-0.5'}"></div>
        </button>
      </div>

      <!-- TUN 模式开关 -->
      <div class="bg-card border border-card-border rounded-xl p-3 flex flex-col justify-between h-24 overflow-hidden">
        <span class="text-sm font-medium text-muted-foreground truncate">TUN 虚拟网卡</span>
        <button
          onclick={() => tunEnabled = !tunEnabled}
          class="w-10 h-6 rounded-full relative transition-colors flex-shrink-0 {tunEnabled ? 'bg-primary' : 'bg-muted'}"
          aria-label={tunEnabled ? '关闭TUN模式' : '开启TUN模式'}
        >
          <div class="w-5 h-5 rounded-full bg-white absolute top-0.5 transition-all shadow {tunEnabled ? 'left-4.5' : 'left-0.5'}"></div>
        </button>
      </div>
    </div>
    <div class="flex-1 overflow-hidden min-h-[200px]">
      <NodeTileGrid nodes={overviewData.proxyNodes} showCheck={true} />
    </div>
  </div>
{/if}
