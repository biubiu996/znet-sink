<script lang="ts">
  import { getToasts, dismissToast, type ToastType } from '$lib/services/toast.svelte';

  const toasts = getToasts();

  function getIcon(type: ToastType): string {
    switch (type) {
      case 'success': return '✓';
      case 'error': return '✕';
      case 'warning': return '⚠';
      case 'info': return 'ℹ';
    }
  }

  function getBorderColor(type: ToastType): string {
    switch (type) {
      case 'success': return 'border-green-500/30';
      case 'error': return 'border-red-500/30';
      case 'warning': return 'border-yellow-500/30';
      case 'info': return 'border-blue-500/30';
    }
  }

  function getIconColor(type: ToastType): string {
    switch (type) {
      case 'success': return 'text-green-500';
      case 'error': return 'text-red-500';
      case 'warning': return 'text-yellow-500';
      case 'info': return 'text-blue-500';
    }
  }
</script>

<div class="fixed bottom-4 right-4 z-50 flex flex-col gap-2 pointer-events-none">
  {#each Array.from(toasts.values()) as toast (toast.id)}
    <div
      class="pointer-events-auto flex items-center gap-3 px-3.5 py-3 bg-card border border-card-border {getBorderColor(toast.type)} rounded-xl shadow-xl min-w-[260px] max-w-[340px] animate-in fade-in slide-in-from-right-6 duration-400 ease-out hover:shadow-2xl transition-all hover:scale-[1.01] backdrop-blur-sm"
    >
      <div class="w-7 h-7 rounded-full bg-muted flex items-center justify-center {getIconColor(toast.type)} text-base font-bold flex-shrink-0">
        {getIcon(toast.type)}
      </div>
      <span class="text-foreground text-xs font-medium flex-1">{toast.message}</span>
      <button
        onclick={() => dismissToast(toast.id)}
        class="text-muted-foreground hover:text-foreground flex-shrink-0 w-5 h-5 rounded-full hover:bg-muted/80 flex items-center justify-center transition-colors"
      >
        <span class="text-xs">✕</span>
      </button>
    </div>
  {/each}
</div>
