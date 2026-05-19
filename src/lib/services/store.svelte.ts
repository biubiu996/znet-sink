import { browser } from '$app/environment';
import type { ThemeMode } from './theme.svelte';
import { getAppConfig, updateAppConfig } from './core';

export type UIMode = 'lite' | 'pro';

class AppStateStore {
  isInitialized = $state(false);
  uiMode = $state<UIMode>('lite');
  activeTab = $state('overview');
  selectedNodeId = $state('node-1');
  selectedTheme = $state<ThemeMode>('system');
  visibleTabs = $state<string[]>(['overview', 'profiles', 'subscriptions', 'rules', 'connections', 'logs', 'capabilities', 'settings']);

  constructor() {
    if (browser) {
      this.hydrateFromLocalStorage();
    }
  }

  private hydrateFromLocalStorage() {
    const savedMode = localStorage.getItem('znet-ui-mode') as UIMode | null;
    const savedInit = localStorage.getItem('znet-is-init');
    const savedTheme = localStorage.getItem('znet-theme') as ThemeMode | null;
    const savedVisibleTabs = localStorage.getItem('znet-visible-tabs');

    if (savedMode) this.uiMode = savedMode;
    if (savedInit === 'true') this.isInitialized = true;
    if (savedTheme) this.selectedTheme = savedTheme;
    if (savedVisibleTabs) this.visibleTabs = JSON.parse(savedVisibleTabs);
  }

  /** Load app config from Rust backend and merge into store state. */
  async loadFromBackend() {
    try {
      const config = await getAppConfig();
      if (config.ui.theme && ['light', 'dark', 'system'].includes(config.ui.theme)) {
        this.selectedTheme = config.ui.theme as ThemeMode;
      }
      if (config.ui.uiMode && ['lite', 'pro'].includes(config.ui.uiMode)) {
        this.uiMode = config.ui.uiMode as UIMode;
      }
      this.isInitialized = true;
      if (browser) {
        localStorage.setItem('znet-is-init', 'true');
      }
    } catch {
      // Backend may not be available yet — use localStorage fallback
    }
  }

  /** Persist theme to Rust backend (and localStorage fallback). */
  async persistTheme(theme: ThemeMode) {
    this.selectedTheme = theme;
    if (browser) {
      localStorage.setItem('znet-theme', theme);
    }
    try {
      await updateAppConfig({ ui: { theme } });
    } catch {
      // Backend may not be available
    }
  }

  startApp(mode: UIMode) {
    this.uiMode = mode;
    this.isInitialized = true;
    if (browser) {
      localStorage.setItem('znet-ui-mode', mode);
      localStorage.setItem('znet-is-init', 'true');
    }
    this.persistUiMode(mode);
  }

  switchUIMode(mode: UIMode) {
    this.uiMode = mode;
    if (mode === 'lite' && (this.activeTab === 'rulesets' || this.activeTab === 'plugins')) {
      this.activeTab = 'overview';
    }
    if (browser) {
      localStorage.setItem('znet-ui-mode', mode);
    }
    this.persistUiMode(mode);
  }

  private async persistUiMode(mode: UIMode) {
    try {
      await updateAppConfig({ ui: { uiMode: mode } });
    } catch {
      // Backend may not be available
    }
  }

  toggleTabVisibility(tabId: string) {
    if (tabId === 'settings') return;
    const index = this.visibleTabs.indexOf(tabId);
    if (index > -1) {
      this.visibleTabs.splice(index, 1);
      if (this.activeTab === tabId) {
        this.activeTab = 'settings';
      }
    } else {
      this.visibleTabs.push(tabId);
    }
    if (browser) {
      localStorage.setItem('znet-visible-tabs', JSON.stringify(this.visibleTabs));
    }
  }

  resetApp() {
    this.isInitialized = false;
    this.activeTab = 'overview';
    this.selectedTheme = 'system';
    this.visibleTabs = ['overview', 'profiles', 'subscriptions', 'rules', 'connections', 'logs', 'capabilities', 'settings'];
    if (browser) {
      localStorage.removeItem('znet-is-init');
      localStorage.removeItem('znet-ui-mode');
      localStorage.removeItem('znet-theme');
      localStorage.removeItem('znet-visible-tabs');
    }
  }
}

export const store = new AppStateStore();
