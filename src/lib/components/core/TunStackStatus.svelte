<script lang="ts">
  import { coreEvents } from '$lib/services/core-events.svelte';
  import { getGuiTunStatus, getGuiStackStatus } from '$lib/services/core';
  import type { GuiFeatureStatus } from '$lib/types/gui-api';
  import { store } from '$lib/services/store.svelte';

  let tunStatus = $state<GuiFeatureStatus | null>(null);
  let stackStatus = $state<GuiFeatureStatus | null>(null);
  let mounted = $state(false);

  const tunLabel = $derived(
    !tunStatus ? '—' :
    coreEvents.tunState === 'started' ? '活跃' :
    coreEvents.tunState === 'error' ? '异常' :
    tunStatus.supported ? '就绪' : '不支持'
  );

  const tunDotColor = $derived(
    coreEvents.tunState === 'started' ? '#22C55E' :
    coreEvents.tunState === 'error' ? '#EF4444' :
    tunStatus?.supported ? '#F59E0B' : 'var(--muted-foreground)'
  );

  const stackLabel = $derived(
    !stackStatus ? '—' :
    coreEvents.stackState === 'started' ? coreEvents.stackMode ?? '运行中' :
    coreEvents.stackState === 'degraded' ? '降级' :
    stackStatus.supported ? '就绪' : '不支持'
  );

  const stackDotColor = $derived(
    coreEvents.stackState === 'started' ? '#22C55E' :
    coreEvents.stackState === 'degraded' ? '#F59E0B' :
    stackStatus?.supported ? '#F59E0B' : 'var(--muted-foreground)'
  );

  async function refresh() {
    try {
      const [tun, stack] = await Promise.all([
        getGuiTunStatus(),
        getGuiStackStatus(),
      ]);
      tunStatus = tun;
      stackStatus = stack;
    } catch {
      // Feature queries may fail if core is not running
    }
  }

  $effect(() => {
    if (store.isInitialized && !mounted) {
      mounted = true;
      refresh();
    }
  });

  // Refresh when core connects
  $effect(() => {
    const tick = coreEvents.statusTick;
    if (tick > 0) refresh();
  });
</script>

<div class="feature-card">
  <div class="feature-header">
    <span class="feature-label">高级功能</span>
  </div>

  <div class="feature-grid">
    <!-- TUN status -->
    <div class="feature-row">
      <div class="feature-dot" style="background: {tunDotColor};"></div>
      <span class="feature-name">TUN 网卡</span>
      <span class="feature-value">{tunLabel}</span>
    </div>

    <!-- Stack status -->
    <div class="feature-row">
      <div class="feature-dot" style="background: {stackDotColor};"></div>
      <span class="feature-name">网络栈</span>
      <span class="feature-value">{stackLabel}</span>
    </div>
  </div>

  {#if coreEvents.tunState === 'error' && coreEvents.tunStateMessage}
    <div class="feature-error">{coreEvents.tunStateMessage}</div>
  {/if}
</div>

<style>
  .feature-card {
    display: flex;
    flex-direction: column;
    gap: 7px;
    min-height: 96px;
    padding: 11px 13px;
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 10px;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.04);
    overflow: hidden;
    transition: box-shadow 0.15s ease, transform 0.15s ease;
  }

  .feature-card:hover {
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.07);
    transform: translateY(-0.5px);
  }

  :global(.dark) .feature-card { box-shadow: 0 1px 3px rgba(0, 0, 0, 0.22); }
  :global(.dark) .feature-card:hover { box-shadow: 0 2px 8px rgba(0, 0, 0, 0.32); }

  .feature-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-shrink: 0;
  }

  .feature-label {
    font-size: 12px;
    font-weight: 500;
    color: var(--muted-foreground);
  }

  .feature-grid {
    display: flex;
    flex-direction: column;
    gap: 6px;
    flex-shrink: 0;
  }

  .feature-row {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .feature-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .feature-name {
    font-size: 11.5px;
    font-weight: 500;
    color: var(--muted-foreground);
    min-width: 52px;
  }

  .feature-value {
    font-size: 11.5px;
    font-weight: 600;
    color: var(--foreground);
    font-variant-numeric: tabular-nums;
  }

  .feature-error {
    font-size: 11px;
    color: var(--destructive);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    flex-shrink: 0;
  }
</style>
