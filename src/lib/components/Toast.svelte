<script lang="ts">
  import { getToasts, dismissToast, type ToastType } from '$lib/services/toast.svelte';

  const toasts = getToasts();

  interface IconPaths {
    success: string;
    error: string;
    warning: string;
    info: string;
  }

  // SVG path data for each type
  const iconPaths: IconPaths = {
    success: 'M2 6l3 3 5-5',         // check
    error:   'M2 2l8 8M10 2L2 10',   // x
    warning: 'M6 3v4M6 9.5v.5',      // exclamation (! body + dot)
    info:    'M6 4v.5M6 6.5v4',      // i body + dot
  };

  function getAccentColor(type: ToastType): string {
    switch (type) {
      case 'success': return 'var(--success)';
      case 'error':   return 'var(--destructive)';
      case 'warning': return 'var(--warning)';
      case 'info':    return 'var(--accent-foreground)';
    }
  }

  function getIconBg(type: ToastType): string {
    switch (type) {
      case 'success': return 'rgba(34,197,94,0.12)';
      case 'error':   return 'rgba(239,68,68,0.12)';
      case 'warning': return 'rgba(245,158,11,0.12)';
      case 'info':    return 'rgba(99,102,241,0.12)';
    }
  }

  function getLabel(type: ToastType): string {
    switch (type) {
      case 'success': return '成功';
      case 'error':   return '错误';
      case 'warning': return '警告';
      case 'info':    return '提示';
    }
  }
</script>

<div class="toast-container">
  {#each Array.from(toasts.values()) as toast (toast.id)}
    <div
      class="toast-item"
      style="--accent: {getAccentColor(toast.type)};"
    >
      <!-- Left accent bar -->
      <div class="toast-bar" style="background: var(--accent);"></div>

      <!-- Icon -->
      <div
        class="toast-icon"
        style="background: {getIconBg(toast.type)}; color: var(--accent);"
      >
        {#if toast.type === 'warning'}
          <!-- warning: exclamation mark -->
          <svg width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round">
            <line x1="6" y1="2.5" x2="6" y2="7"/>
            <circle cx="6" cy="9.5" r="0.6" fill="currentColor" stroke="none"/>
          </svg>
        {:else if toast.type === 'info'}
          <!-- info: letter i -->
          <svg width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round">
            <circle cx="6" cy="2.5" r="0.6" fill="currentColor" stroke="none"/>
            <line x1="6" y1="4.5" x2="6" y2="9.5"/>
          </svg>
        {:else if toast.type === 'error'}
          <!-- error: × -->
          <svg width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round">
            <line x1="2.5" y1="2.5" x2="9.5" y2="9.5"/>
            <line x1="9.5" y1="2.5" x2="2.5" y2="9.5"/>
          </svg>
        {:else}
          <!-- success: ✓ -->
          <svg width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="2,6 5,9 10,3"/>
          </svg>
        {/if}
      </div>

      <!-- Text -->
      <div class="toast-text">
        <span class="toast-label" style="color: var(--accent);">{getLabel(toast.type)}</span>
        <span class="toast-msg">{toast.message}</span>
      </div>

      <!-- Dismiss -->
      <button
        onclick={() => dismissToast(toast.id)}
        class="toast-dismiss"
        aria-label="关闭"
      >
        <svg width="10" height="10" viewBox="0 0 10 10" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round">
          <line x1="2" y1="2" x2="8" y2="8"/>
          <line x1="8" y1="2" x2="2" y2="8"/>
        </svg>
      </button>
    </div>
  {/each}
</div>

<style>
  .toast-container {
    position: fixed;
    bottom: 16px;
    right: 16px;
    z-index: 9999;
    display: flex;
    flex-direction: column;
    gap: 6px;
    pointer-events: none;
  }

  .toast-item {
    pointer-events: auto;
    position: relative;
    display: flex;
    align-items: center;
    gap: 9px;
    padding: 9px 10px 9px 14px;
    min-width: 240px;
    max-width: 320px;
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 9px;
    overflow: hidden;
    box-shadow:
      0 4px 16px rgba(0, 0, 0, 0.1),
      0 1px 4px rgba(0, 0, 0, 0.06);
    animation: toast-in 0.2s cubic-bezier(0.22, 1, 0.36, 1);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
  }

  :global(.dark) .toast-item {
    box-shadow:
      0 4px 20px rgba(0, 0, 0, 0.4),
      0 1px 4px rgba(0, 0, 0, 0.3);
  }

  /* Left accent bar */
  .toast-bar {
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    width: 3px;
    border-radius: 0 2px 2px 0;
  }

  .toast-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    border-radius: 6px;
    flex-shrink: 0;
  }

  .toast-text {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .toast-label {
    font-size: 12px;
    font-weight: 600;
    letter-spacing: 0.02em;
    line-height: 1;
  }

  .toast-msg {
    font-size: 12px;
    font-weight: 400;
    color: var(--foreground);
    line-height: 1.35;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .toast-dismiss {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 20px;
    height: 20px;
    border-radius: 4px;
    background: transparent;
    border: none;
    cursor: pointer;
    color: var(--muted-foreground);
    flex-shrink: 0;
    transition: background 0.12s ease, color 0.12s ease;
  }

  .toast-dismiss:hover {
    background: var(--muted);
    color: var(--foreground);
  }

  @keyframes toast-in {
    from {
      opacity: 0;
      transform: translateX(16px) scale(0.97);
    }
    to {
      opacity: 1;
      transform: translateX(0) scale(1);
    }
  }
</style>
