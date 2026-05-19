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

<div class="bg-card border border-card-border rounded-xl p-4">
  <h3 class="text-sm font-bold text-foreground mb-4">内核配置</h3>

  {#if !snapshot}
    <div class="text-xs text-muted-foreground">加载中...</div>
  {:else}
    <div class="space-y-3 text-xs">
      <div class="flex justify-between items-center">
        <span class="text-muted-foreground">内核</span>
        <span class="font-mono text-foreground truncate max-w-[200px]">{snapshot.kernel}</span>
      </div>
      <div class="flex justify-between items-center">
        <span class="text-muted-foreground">可执行文件</span>
        <span class="font-mono text-foreground truncate max-w-[200px]">{snapshot.executablePath ?? '-'}</span>
      </div>
      <div class="flex justify-between items-center">
        <span class="text-muted-foreground">文件存在</span>
        <span class="font-mono {snapshot.executableExists ? 'text-green-500' : 'text-yellow-500'}">{snapshot.executableExists ? '是' : '否'}</span>
      </div>
      <div class="flex justify-between items-center">
        <span class="text-muted-foreground">配置文件</span>
        <span class="font-mono text-foreground truncate max-w-[200px]">{snapshot.configPath ?? '-'}</span>
      </div>
      <div class="flex justify-between items-center">
        <span class="text-muted-foreground">工作目录</span>
        <span class="font-mono text-foreground truncate max-w-[200px]">{snapshot.workingDir ?? '-'}</span>
      </div>
      <div class="flex justify-between items-center">
        <span class="text-muted-foreground">Socket</span>
        <span class="font-mono text-foreground truncate max-w-[200px]">{snapshot.endpoint.path}</span>
      </div>
      <div class="flex justify-between items-center">
        <span class="text-muted-foreground">启动参数</span>
        <span class="font-mono text-foreground truncate max-w-[200px]">{snapshot.launchArgs.join(' ')}</span>
      </div>
      {#if snapshot.warnings.length > 0}
        <div class="mt-2">
          <span class="text-xs text-yellow-500 font-medium">警告</span>
          {#each snapshot.warnings as warning}
            <div class="text-[10px] text-yellow-500 mt-1">{warning}</div>
          {/each}
        </div>
      {/if}
    </div>
  {/if}
</div>
