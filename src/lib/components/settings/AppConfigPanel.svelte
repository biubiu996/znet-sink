<script lang="ts">
  import { getAppConfig, updateAppConfig } from '$lib/services/core';
  import { store } from '$lib/services/store.svelte';
  import { setTheme, type ThemeMode } from '$lib/services/theme.svelte';
  import type { AppConfig } from '$lib/types/app-config';

  let config = $state<AppConfig | null>(null);
  let loading = $state(false);

  async function refreshConfig() {
    try {
      config = await getAppConfig();
    } catch (e) {
      console.error('Failed to get app config:', e);
    }
  }

  async function toggleCoreSetting(key: 'autoStart' | 'autoConnect') {
    if (!config) return;
    loading = true;
    try {
      const current = config.core[key];
      const updated = await updateAppConfig({ core: { [key]: !current } });
      config = updated;
    } catch (e) {
      console.error('Failed to update config:', e);
    } finally {
      loading = false;
    }
  }

  function handleThemeChange(theme: ThemeMode) {
    setTheme(theme);
  }

  $effect(() => {
    refreshConfig();
  });
</script>

<div class="bg-card border border-card-border rounded-xl p-4">
  <h3 class="text-sm font-bold text-foreground mb-4">应用配置</h3>

  {#if !config}
    <div class="text-xs text-muted-foreground">加载中...</div>
  {:else}
    <div class="space-y-4">
      <!-- 主题设置 -->
      <div class="flex items-center justify-between">
        <span class="text-xs text-muted-foreground">主题</span>
        <div class="flex bg-muted rounded-lg p-0.5 text-[10px] font-bold">
          {#each ['light', 'dark', 'system'] as theme}
            <button
              onclick={() => handleThemeChange(theme as ThemeMode)}
              class="px-3 py-1 rounded-md transition-all {store.selectedTheme === theme ? 'bg-primary text-primary-foreground' : 'text-muted-foreground hover:text-foreground'}"
            >
              {theme === 'light' ? '明亮' : theme === 'dark' ? '暗黑' : '跟随系统'}
            </button>
          {/each}
        </div>
      </div>

      <!-- UI模式 -->
      <div class="flex items-center justify-between">
        <span class="text-xs text-muted-foreground">界面模式</span>
        <div class="flex bg-muted rounded-lg p-0.5 text-[10px] font-bold">
          <button
            onclick={() => store.switchUIMode('lite')}
            class="px-3 py-1 rounded-md transition-all {store.uiMode === 'lite' ? 'bg-primary text-primary-foreground' : 'text-muted-foreground hover:text-foreground'}"
          >
            简约
          </button>
          <button
            onclick={() => store.switchUIMode('pro')}
            class="px-3 py-1 rounded-md transition-all {store.uiMode === 'pro' ? 'bg-primary text-primary-foreground' : 'text-muted-foreground hover:text-foreground'}"
          >
            专业
          </button>
        </div>
      </div>

      <!-- 自动启动内核 -->
      <div class="flex items-center justify-between">
        <span class="text-xs text-muted-foreground">开机自动启动内核</span>
        <button
          onclick={() => toggleCoreSetting('autoStart')}
          disabled={loading}
          class="w-9 h-5 rounded-full relative transition-colors disabled:opacity-50 {config.core.autoStart ? 'bg-primary' : 'bg-muted'}"
        >
          <div class="w-4 h-4 rounded-full bg-white absolute top-0.5 transition-all shadow {config.core.autoStart ? 'left-4' : 'left-0.5'}"></div>
        </button>
      </div>

      <!-- 自动连接 -->
      <div class="flex items-center justify-between">
        <span class="text-xs text-muted-foreground">启动后自动连接</span>
        <button
          onclick={() => toggleCoreSetting('autoConnect')}
          disabled={loading}
          class="w-9 h-5 rounded-full relative transition-colors disabled:opacity-50 {config.core.autoConnect ? 'bg-primary' : 'bg-muted'}"
        >
          <div class="w-4 h-4 rounded-full bg-white absolute top-0.5 transition-all shadow {config.core.autoConnect ? 'left-4' : 'left-0.5'}"></div>
        </button>
      </div>

      <!-- 页面可见性 -->
      <div>
        <div class="text-xs text-muted-foreground mb-2">页面可见性</div>
        <div class="flex flex-wrap gap-1">
          {#each [
            { id: 'overview', label: '概览' },
            { id: 'profiles', label: '配置' },
            { id: 'subscriptions', label: '订阅' },
            { id: 'rules', label: '规则' },
            { id: 'connections', label: '连接' },
            { id: 'logs', label: '日志' },
            { id: 'settings', label: '设置' }
          ] as tab}
            <button
              onclick={() => tab.id !== 'settings' && store.toggleTabVisibility(tab.id)}
              class="px-2 py-1 rounded text-[10px] font-bold transition-all whitespace-nowrap
                     {store.visibleTabs.includes(tab.id)
                       ? 'bg-primary text-primary-foreground shadow-sm'
                       : 'bg-muted text-muted-foreground hover:text-foreground'}
                     {tab.id === 'settings' ? 'cursor-not-allowed' : ''}"
            >
              {tab.label}
            </button>
          {/each}
        </div>
      </div>
    </div>
  {/if}
</div>
