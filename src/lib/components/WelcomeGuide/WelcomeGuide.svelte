<script lang="ts">
  import { scale, fly, fade } from 'svelte/transition';
  import { elasticOut, cubicOut } from 'svelte/easing';
  import { store } from '$lib/services/store.svelte';
</script>

<section
  transition:fade={{ duration: 300 }}
  class="flex-1 w-full flex flex-col items-center justify-center"
>
  <!-- Brand mark -->
  <div
    transition:scale={{ delay: 80, duration: 500, easing: elasticOut, start: 0.85 }}
    class="text-center mb-8"
  >
    <div class="welcome-icon">
      <img src="/favicon.png" alt="ZNet Sink" style="width: 36px; height: 36px;" class="rounded-xl opacity-90" />
    </div>
    <h2 class="welcome-title">ZNet Sink</h2>
    <p class="welcome-sub">请选择适合您的工作流模式</p>
  </div>

  <!-- Mode cards -->
  <div class="welcome-cards">
    <button
      transition:fly={{ delay: 160, x: -20, duration: 380, easing: cubicOut }}
      onclick={() => store.startApp('lite')}
      class="mode-card"
    >
      <div class="mode-card-header">
        <span class="mode-badge">Lite</span>
        <span class="mode-title">简约模式</span>
      </div>
      <p class="mode-desc">启动开关、订阅管理、节点切换。极简开箱即用。</p>
    </button>

    <button
      transition:fly={{ delay: 220, x: 20, duration: 380, easing: cubicOut }}
      onclick={() => store.startApp('pro')}
      class="mode-card mode-card-pro"
    >
      <div class="mode-card-header">
        <span class="mode-badge pro">Pro</span>
        <span class="mode-title">专业模式</span>
      </div>
      <p class="mode-desc">全能力开放，解锁流量走势、连接树、规则重写及插件扩展。</p>
    </button>
  </div>

  <!-- Hint -->
  <div
    transition:fly={{ delay: 360, y: 12, duration: 300, easing: cubicOut }}
    class="welcome-hint"
  >
    <svg width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round" opacity="0.6">
      <circle cx="6" cy="6" r="5"/>
      <line x1="6" y1="4" x2="6" y2="6"/>
      <circle cx="6" cy="8" r="0.5" fill="currentColor"/>
    </svg>
    <span>启动后可在「设置 → 通用」随时切换模式</span>
  </div>
</section>

<style>
  .welcome-icon {
    width: 56px;
    height: 56px;
    border-radius: 14px;
    background: var(--card);
    border: 1px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: center;
    margin: 0 auto 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.07);
  }

  :global(.dark) .welcome-icon {
    box-shadow: 0 2px 12px rgba(0, 0, 0, 0.3);
  }

  .welcome-title {
    font-size: 16px;
    font-weight: 700;
    color: var(--foreground);
    letter-spacing: -0.02em;
    margin-bottom: 4px;
  }

  .welcome-sub {
    font-size: 12.5px;
    color: var(--muted-foreground);
    font-weight: 400;
  }

  /* ---- Cards ---- */
  .welcome-cards {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 10px;
    width: 440px;
    max-width: 100%;
  }

  .mode-card {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: 16px 18px;
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 10px;
    text-align: left;
    cursor: pointer;
    transition: all 0.15s ease;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
  }

  .mode-card:hover {
    border-color: rgba(0, 0, 0, 0.12);
    box-shadow: 0 3px 10px rgba(0, 0, 0, 0.1);
    transform: translateY(-1px);
  }

  :global(.dark) .mode-card:hover {
    border-color: rgba(255, 255, 255, 0.12);
    box-shadow: 0 3px 12px rgba(0, 0, 0, 0.3);
  }

  .mode-card-pro:hover {
    border-color: rgba(99, 102, 241, 0.3);
  }

  :global(.dark) .mode-card-pro:hover {
    border-color: rgba(165, 180, 252, 0.25);
  }

  .mode-card-header {
    display: flex;
    align-items: center;
    gap: 7px;
  }

  .mode-badge {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    height: 19px;
    padding: 0 6px;
    border-radius: 4px;
    font-size: 11.5px;
    font-weight: 700;
    letter-spacing: 0.04em;
    background: var(--muted);
    color: var(--muted-foreground);
    flex-shrink: 0;
  }

  .mode-badge.pro {
    background: rgba(99, 102, 241, 0.1);
    color: #6366F1;
  }

  :global(.dark) .mode-badge.pro {
    background: rgba(165, 180, 252, 0.12);
    color: #A5B4FC;
  }

  .mode-title {
    font-size: 13px;
    font-weight: 600;
    color: var(--foreground);
  }

  .mode-desc {
    font-size: 11.5px;
    color: var(--muted-foreground);
    line-height: 1.55;
    text-align: left;
  }

  /* ---- Hint ---- */
  .welcome-hint {
    display: flex;
    align-items: center;
    gap: 5px;
    margin-top: 14px;
    font-size: 11.5px;
    color: var(--muted-foreground);
    opacity: 0.75;
  }
</style>
