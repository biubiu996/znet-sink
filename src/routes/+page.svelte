<script lang="ts">
  import { onMount } from 'svelte';
  import { store } from '$lib/services/store.svelte';
  import { guiState } from '$lib/services/gui-state.svelte';
  import { coreEvents } from '$lib/services/core-events.svelte';
  import { initTheme, applyTheme } from '$lib/services/theme.svelte';
  import TitleBar from '$lib/components/TitleBar.svelte';
  import AppHeader from '$lib/components/AppHeader.svelte';
  import TabContent from '$lib/components/TabContent.svelte';
  import { WelcomeGuide } from '$lib/components/WelcomeGuide';
  import Toast from '$lib/components/Toast.svelte';

  onMount(() => {
    initTheme();
    void store.loadFromBackend();
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    const onSystemThemeChange = () => {
      if (store.selectedTheme === 'system') applyTheme('system');
    };
    mediaQuery.addEventListener('change', onSystemThemeChange);
    return () => mediaQuery.removeEventListener('change', onSystemThemeChange);
  });

  $effect(() => {
    if (store.isInitialized) {
      guiState.initialize();
      coreEvents.start();
    } else {
      guiState.destroy();
      coreEvents.stop();
    }
    return () => {
      guiState.destroy();
      coreEvents.stop();
    };
  });
</script>

<main class="h-screen w-screen flex flex-col select-none overflow-hidden transition-colors duration-200"
  style="background: var(--background); color: var(--foreground); font-family: var(--font-sans, system-ui);">

  <!-- Title bar: 44px, drag region -->
  <TitleBar />

  <!-- Nav header: 38px -->
  <div class="flex-shrink-0 px-5 pt-2.5">
    <AppHeader />
  </div>

  <!-- Separator -->
  <div class="flex-shrink-0 mx-5" style="height: 1px; background: var(--border); opacity: 0.5;"></div>

  <!-- Main content area -->
  <div class="flex-1 min-h-0 px-3 sm:px-5 py-2 sm:py-3.5 flex flex-col overflow-hidden">
    {#if store.appLoading}
      <div class="flex-1 flex items-center justify-center">
        <span style="font-size: 13px; color: var(--muted-foreground); opacity: 0.5;">加载中…</span>
      </div>
    {:else if store.loadError}
      <div class="flex-1 flex flex-col items-center justify-center gap-3">
        <span style="font-size: 14px; color: var(--destructive); font-weight: 600;">启动失败</span>
        <span style="font-size: 12px; color: var(--muted-foreground); max-width: 360px; text-align: center;">{store.loadError}</span>
        <button
          class="retry-btn"
          onclick={() => { store.loadError = null; store.appLoading = true; store.loadFromBackend(); }}
        >重试</button>
      </div>
    {:else if !store.isInitialized}
      <WelcomeGuide />
    {:else}
      <TabContent />
    {/if}
  </div>

  <Toast />
</main>
