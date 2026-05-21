<script lang="ts">
  import { getAppConfig, updateAppConfig } from '$lib/services/core';
  import { store } from '$lib/services/store.svelte';
  import { setTheme, type ThemeMode } from '$lib/services/theme.svelte';
  import type { AppConfig } from '$lib/types/app-config';
  import { Switch } from '$lib/components/ui/switch';

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

  const THEMES: Array<{ value: ThemeMode; label: string }> = [
    { value: 'light',  label: '明亮' },
    { value: 'dark',   label: '暗黑' },
    { value: 'system', label: '跟随系统' },
  ];

  $effect(() => {
    refreshConfig();
  });
</script>

<div class="config-section">
  <div class="config-section-title">外观</div>

  <!-- Theme -->
  <div class="config-row">
    <div class="config-row-label">
      <span class="label-text">主题</span>
      <span class="label-desc">选择界面配色方案</span>
    </div>
    <div class="theme-segment">
      {#each THEMES as theme}
        <button
          onclick={() => handleThemeChange(theme.value)}
          class="theme-seg-btn {store.selectedTheme === theme.value ? 'active' : ''}"
          aria-pressed={store.selectedTheme === theme.value}
        >
          {theme.label}
        </button>
      {/each}
    </div>
  </div>

  <!-- UI Mode -->
  <div class="config-row">
    <div class="config-row-label">
      <span class="label-text">界面模式</span>
      <span class="label-desc">简约模式隐藏高级功能</span>
    </div>
    <div class="theme-segment">
      <button
        onclick={async () => await store.switchUIMode('lite')}
        class="theme-seg-btn {store.uiMode === 'lite' ? 'active' : ''}"
      >
        简约
      </button>
      <button
        onclick={async () => await store.switchUIMode('pro')}
        class="theme-seg-btn {store.uiMode === 'pro' ? 'active' : ''}"
      >
        专业
      </button>
    </div>
  </div>
</div>

<div class="config-separator"></div>

<div class="config-section">
  <div class="config-section-title">内核行为</div>

  {#if !config}
    <div class="config-loading">加载配置中…</div>
  {:else}
    <!-- Auto start -->
    <div class="config-row">
      <div class="config-row-label">
        <span class="label-text">开机自动启动内核</span>
        <span class="label-desc">系统启动时自动运行内核进程</span>
      </div>
      <Switch
        checked={config.core.autoStart}
        onCheckedChange={() => toggleCoreSetting('autoStart')}
        disabled={loading}
        aria-label="开机自动启动内核"
      />
    </div>

    <!-- Auto connect -->
    <div class="config-row">
      <div class="config-row-label">
        <span class="label-text">启动后自动连接</span>
        <span class="label-desc">内核启动完成后自动建立连接</span>
      </div>
      <Switch
        checked={config.core.autoConnect}
        onCheckedChange={() => toggleCoreSetting('autoConnect')}
        disabled={loading}
        aria-label="启动后自动连接"
      />
    </div>
  {/if}
</div>

<style>
  /* ---- Section ---- */
  .config-section {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .config-section-title {
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.07em;
    text-transform: uppercase;
    color: var(--muted-foreground);
    padding: 0 0 8px;
    opacity: 0.7;
  }

  .config-separator {
    height: 1px;
    background: var(--border);
    margin: 16px 0;
  }

  /* ---- Row ---- */
  .config-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 10px 0;
    border-bottom: 1px solid var(--border);
  }

  .config-row:last-child {
    border-bottom: none;
  }

  .config-row-label {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
    flex: 1;
  }

  .label-text {
    font-size: 13px;
    font-weight: 500;
    color: var(--foreground);
  }

  .label-desc {
    font-size: 11.5px;
    color: var(--muted-foreground);
    opacity: 0.8;
  }

  /* ---- Theme segmented ---- */
  .theme-segment {
    display: inline-flex;
    align-items: center;
    gap: 1px;
    background: var(--segment-bg, rgba(0,0,0,0.055));
    padding: 2px;
    border-radius: 7px;
    flex-shrink: 0;
  }

  .theme-seg-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    height: 26px;
    padding: 0 11px;
    border-radius: 5px;
    border: none;
    background: transparent;
    color: var(--muted-foreground);
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.13s ease;
    white-space: nowrap;
  }

  .theme-seg-btn:hover {
    color: var(--foreground);
  }

  .theme-seg-btn.active {
    background: var(--segment-active-bg, #ffffff);
    box-shadow: var(--segment-active-shadow, 0 1px 3px rgba(0,0,0,0.12));
    color: var(--foreground);
    font-weight: 600;
  }

  /* ---- Loading ---- */
  .config-loading {
    font-size: 12px;
    color: var(--muted-foreground);
    padding: 14px 0;
    text-align: center;
    opacity: 0.6;
  }
</style>
