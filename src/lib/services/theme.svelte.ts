import { browser } from '$app/environment';
import { store } from './store.svelte';

export type ThemeMode = 'light' | 'dark' | 'system';

export function applyTheme(mode: ThemeMode) {
  if (!browser) return;

  const root = document.documentElement;
  if (mode === 'system') {
    const isDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    root.classList.toggle('dark', isDark);
  } else {
    root.classList.toggle('dark', mode === 'dark');
  }
}

export function initTheme() {
  if (!browser) return;
  const saved = (localStorage.getItem('znet-theme') as ThemeMode | null) || store.selectedTheme;
  applyTheme(saved || 'system');
}

export function setTheme(mode: ThemeMode) {
  applyTheme(mode);
  store.persistTheme(mode);
}
