<script lang="ts">
  import { getCoreProcessStatus, startCoreProcess, stopCoreProcess, getCoreConfigSnapshot } from '$lib/services/core';
  import type { CoreProcessStatus, CoreConfigSnapshot } from '$lib/types/core';

  let status = $state<CoreProcessStatus | null>(null);
  let snapshot = $state<CoreConfigSnapshot | null>(null);
  let loading = $state(false);

  const isRunning = $derived(status?.state === 'running');
  const isStarting = $derived(status?.state === 'starting');
  const isStopped = $derived(status?.exitReason === 'stopped');
  const isCrashed = $derived(status?.exitReason === 'crashed');
  const hasFailed = $derived(status?.state === 'failed');

  async function refreshStatus() {
    try {
      status = await getCoreProcessStatus();
    } catch (e) {
      console.error('Failed to get core status:', e);
    }
  }

  async function validateConfig() {
    try {
      snapshot = await getCoreConfigSnapshot();
    } catch {
      snapshot = null;
    }
  }

  async function toggleCore() {
    if (loading) return;
    loading = true;
    try {
      if (isRunning) {
        await stopCoreProcess();
      } else {
        // Pre-launch validation
        await validateConfig();
        if (snapshot?.warnings.length) {
          const proceed = confirm(
            `内核配置存在以下警告:\n\n${snapshot.warnings.map(w => '• ' + w).join('\n')}\n\n是否仍然启动？`
          );
          if (!proceed) { loading = false; return; }
        }
        await startCoreProcess();
      }
      await refreshStatus();
    } catch (e) {
      console.error('Failed to toggle core:', e);
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    refreshStatus();
    validateConfig();
    const interval = setInterval(refreshStatus, 5000);
    return () => clearInterval(interval);
  });
</script>

<div class="bg-card border border-card-border rounded-xl p-3 flex flex-col gap-2 h-24 overflow-hidden">
  <div class="flex items-center justify-between flex-shrink-0">
    <span class="text-sm font-medium text-muted-foreground truncate">内核状态</span>
    <div class="flex items-center gap-1.5 flex-shrink-0">
      <div class="w-2.5 h-2.5 rounded-full
        {isRunning ? 'bg-green-500' : isStarting ? 'bg-yellow-500 animate-pulse' : hasFailed || isCrashed ? 'bg-red-500' : 'bg-muted'}">
      </div>
      <span class="text-sm font-bold text-foreground">
        {isRunning ? '运行中' : isStarting ? '启动中' : hasFailed ? '启动失败' : isCrashed ? '异常退出' : '已停止'}
      </span>
    </div>
  </div>

  {#if isRunning && status}
    <div class="grid grid-cols-2 gap-1 text-xs flex-shrink-0">
      <div class="flex justify-between overflow-hidden">
        <span class="text-muted-foreground truncate">PID</span>
        <span class="font-mono text-foreground truncate ml-1">{status.pid ?? '-'}</span>
      </div>
      <div class="flex justify-between overflow-hidden">
        <span class="text-muted-foreground truncate">内核</span>
        <span class="font-mono text-foreground truncate ml-1">{status.kernel}</span>
      </div>
    </div>
  {:else if status?.exitReason && status.state === 'exited'}
    <div class="grid grid-cols-2 gap-1 text-xs flex-shrink-0">
      <div class="flex justify-between overflow-hidden">
        <span class="text-muted-foreground truncate">退码</span>
        <span class="font-mono text-foreground truncate ml-1">{status.exitCode ?? '-'}</span>
      </div>
      <div class="flex justify-between overflow-hidden">
        <span class="text-muted-foreground truncate">原因</span>
        <span class="font-mono truncate ml-1 {isCrashed ? 'text-red-500' : 'text-muted-foreground'}">
          {isStopped ? '手动停止' : isCrashed ? '崩溃' : '自行退出'}
        </span>
      </div>
      {#if isCrashed && status.lastError}
        <div class="col-span-2 flex justify-between overflow-hidden">
          <span class="text-muted-foreground truncate">错误</span>
          <span class="text-red-500 truncate ml-1 text-[10px]">{status.lastError}</span>
        </div>
      {/if}
    </div>
  {:else if hasFailed && status?.lastError}
    <div class="flex justify-between overflow-hidden text-xs flex-shrink-0">
      <span class="text-muted-foreground truncate">错误</span>
      <span class="text-red-500 truncate ml-1">{status.lastError}</span>
    </div>
  {/if}

  {#if snapshot?.warnings.length && !isRunning && !isStarting}
    <div class="flex items-center gap-1 text-[10px] text-yellow-500 flex-shrink-0" title={snapshot.warnings.join('; ')}>
      <span class="truncate">⚠ {snapshot.warnings[0]}</span>
    </div>
  {/if}

  <button
    onclick={toggleCore}
    disabled={loading || isStarting}
    class="w-full py-1.5 rounded-lg font-medium text-xs transition-all disabled:opacity-50 mt-auto flex-shrink-0 truncate
           {isRunning
             ? 'bg-red-500/10 text-red-500 hover:bg-red-500/20 border border-red-500/30'
             : 'bg-green-500/10 text-green-500 hover:bg-green-500/20 border border-green-500/30'}"
  >
    {loading || isStarting ? '处理中...' : isRunning ? '停止内核' : '启动内核'}
  </button>
</div>
