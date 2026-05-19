<script lang="ts">
  import { onMount } from 'svelte';
  import { store } from '$lib/services/store.svelte';
  import { overviewData } from '$lib/services/overview-data.svelte';
  import { coreEvents } from '$lib/services/core-events.svelte';
  import { getAppConfig, enableSystemProxy } from '$lib/services/core';
  import { initTheme, applyTheme } from '$lib/services/theme.svelte';
  import TitleBar from '$lib/components/TitleBar.svelte';
  import AppHeader from '$lib/components/AppHeader.svelte';
  import TabContent from '$lib/components/TabContent.svelte';
  import WelcomeGuide from '$lib/components/WelcomeGuide.svelte';

  onMount(() => {
    initTheme();
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    const onSystemThemeChange = () => {
      if (store.selectedTheme === 'system') applyTheme('system');
    };
    mediaQuery.addEventListener('change', onSystemThemeChange);
    return () => mediaQuery.removeEventListener('change', onSystemThemeChange);
  });

  async function tryAutoConnect() {
    try {
      const config = await getAppConfig();
      if (config.core.autoConnect) {
        await enableSystemProxy();
      }
    } catch { /* best-effort: core may not be running yet */ }
  }

  $effect(() => {
    if (store.isInitialized) {
      overviewData.start();
      coreEvents.start();
      tryAutoConnect();
    } else {
      overviewData.stop();
      coreEvents.stop();
    }
    return () => {
      overviewData.stop();
      coreEvents.stop();
    };
  });
</script>

<main class="h-screen w-screen bg-background text-foreground flex flex-col select-none font-sans text-xs overflow-hidden transition-colors duration-300">
  <TitleBar />

  <div class="flex-1 w-full p-4 flex flex-col gap-3 overflow-hidden">
    <AppHeader />

    <div class="flex-1 w-full overflow-hidden flex flex-col gap-3">
      {#if !store.isInitialized}
        <WelcomeGuide />
      {:else}
        <TabContent />
      {/if}
    </div>
  </div>
</main>
