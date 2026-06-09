<script lang="ts">
  import { configEditor, type ValidationError, type ConfigImpactItem } from '$lib/services/config-editor.svelte';
  import { Button } from '$lib/components/ui/button';
  import { AlertTriangle, Check, Loader2, RefreshCcw, RotateCcw, Send, ScanSearch, Zap, Power } from '@lucide/svelte';

  let textareaRef: HTMLTextAreaElement | undefined = $state();
  let tabSize = 2;

  $effect(() => {
    // Auto-load on mount
    if (configEditor.phase === 'idle') {
      configEditor.load();
    }
  });

  function handleInput(e: Event) {
    const target = e.target as HTMLTextAreaElement;
    configEditor.updateDraft(target.value);
  }

  function handleKeyDown(e: KeyboardEvent) {
    // Tab inserts spaces instead of changing focus
    if (e.key === 'Tab') {
      e.preventDefault();
      const textarea = textareaRef;
      if (!textarea) return;
      const start = textarea.selectionStart;
      const end = textarea.selectionEnd;
      const spaces = ' '.repeat(tabSize);
      const value = textarea.value;
      const newValue = value.substring(0, start) + spaces + value.substring(end);
      textarea.value = newValue;
      textarea.selectionStart = textarea.selectionEnd = start + tabSize;
      configEditor.updateDraft(newValue);
    }
  }

  async function handleLoad() {
    await configEditor.load();
  }

  async function handleValidate() {
    await configEditor.validate();
  }

  async function handlePlanApply() {
    await configEditor.planApply();
  }

  function handleReset() {
    configEditor.reset();
  }

  function formatJson() {
    const parsed = configEditor.parseDraft();
    if (parsed) {
      configEditor.updateDraft(JSON.stringify(parsed, null, 2));
    }
  }

  function jsonLineCount(): number {
    const lines = configEditor.draftJson.split('\n');
    return Math.max(lines.length, 1);
  }

  /** Section label mapping for human-friendly display. */
  const sectionLabels: Record<string, string> = {
    outbounds: '出站代理',
    listeners: '监听器',
    rules: '路由规则',
    tun: 'TUN 虚拟网卡',
    dns: 'DNS',
    inbound: '入站',
    routing: '路由',
    experimental: '实验选项',
    config: '配置',
  };

  function sectionLabel(section: string): string {
    return sectionLabels[section] ?? section;
  }

  const isLoading = $derived(
    configEditor.phase === 'validating' || configEditor.phase === 'applying' || configEditor.phase === 'planning'
  );
  const canValidate = $derived(
    configEditor.phase === 'loaded' || configEditor.phase === 'editing' || configEditor.phase === 'error'
  );
  const canPlanApply = $derived(
    configEditor.supportsPlanApply
    && (configEditor.phase === 'editing' || configEditor.phase === 'loaded' || configEditor.phase === 'error')
    && configEditor.dirty
  );
  const canApply = $derived(
    (configEditor.phase === 'editing' || configEditor.phase === 'loaded' || configEditor.phase === 'planned') && configEditor.dirty
  );
  const canReset = $derived(configEditor.dirty);
  const hasValidationErrors = $derived(configEditor.validationErrors.length > 0);
  const hasPlanResult = $derived(configEditor.planResult !== null);
  const hasRestartImpact = $derived(
    configEditor.planResult !== null && configEditor.planResult.requiresRestart.length > 0
  );
  /** Whether the "应用" button should act as a "confirm after restart-impact review". */
  const needsConfirmApply = $derived(
    configEditor.phase === 'planned' && hasRestartImpact
  );

  async function handleApply() {
    if (needsConfirmApply) {
      await configEditor.confirmApply();
    } else {
      await configEditor.apply();
    }
  }
</script>

<div class="panel">
  <!-- Header -->
  <div class="header">
    <div class="heading">
      <div class="title">内核配置编辑</div>
      <div class="desc">
        编辑运行中内核的配置草稿。保存前自动校验，应用后自动对账。修改不会立即生效，直到点击「应用」。
      </div>
    </div>
    <div class="actions">
      <Button variant="ghost" size="icon-sm" onclick={handleLoad} disabled={isLoading}>
        <RefreshCcw class="h-3.5 w-3.5" />
      </Button>
    </div>
  </div>

  <!-- Status bar -->
  <div class="status-bar">
    <div class="status-left">
      {#if configEditor.phase === 'idle' || configEditor.phase === 'loaded'}
        <span class="status-badge {configEditor.phase === 'loaded' ? 'ready' : ''}">
          {configEditor.phase === 'loaded' ? '已加载' : '未加载'}
        </span>
      {:else if configEditor.phase === 'editing'}
        <span class="status-badge editing">编辑中</span>
      {:else if configEditor.phase === 'validating'}
        <span class="status-badge loading">
          <Loader2 class="h-3 w-3 animate-spin" />
          校验中
        </span>
      {:else if configEditor.phase === 'planning'}
        <span class="status-badge loading">
          <Loader2 class="h-3 w-3 animate-spin" />
          预检中
        </span>
      {:else if configEditor.phase === 'planned'}
        <span class="status-badge planned">
          <ScanSearch class="h-3 w-3" />
          {hasRestartImpact ? '部分需重启' : '可热加载'}
        </span>
      {:else if configEditor.phase === 'applying'}
        <span class="status-badge loading">
          <Loader2 class="h-3 w-3 animate-spin" />
          应用中
        </span>
      {:else if configEditor.phase === 'applied'}
        <span class="status-badge success">
          <Check class="h-3 w-3" />
          已应用
        </span>
      {:else if configEditor.phase === 'error'}
        <span class="status-badge error">
          <AlertTriangle class="h-3 w-3" />
          错误
        </span>
      {/if}

      {#if configEditor.dirty}
        <span class="dirty-badge">未保存</span>
      {/if}

      {#if configEditor.lastAppliedAt}
        <span class="applied-time">
          上次应用: {new Date(configEditor.lastAppliedAt).toLocaleTimeString('zh-CN', { hour12: false })}
        </span>
      {/if}
    </div>

    <div class="actions-bar">
      <Button
        variant="outline"
        size="sm"
        onclick={formatJson}
        disabled={!configEditor.draftJson}
        title="格式化 JSON"
      >
        格式化
      </Button>
      <Button
        variant="outline"
        size="sm"
        onclick={handleReset}
        disabled={!canReset}
        title="放弃修改，恢复到内核当前配置"
      >
        <RotateCcw class="h-3.5 w-3.5" />
        重置
      </Button>
      <Button
        variant="outline"
        size="sm"
        onclick={handleValidate}
        disabled={!canValidate || isLoading}
        title="校验配置，不应用到内核"
      >
        {#if configEditor.phase === 'validating'}
          <Loader2 class="h-3.5 w-3.5 animate-spin" />
        {:else}
          <Check class="h-3.5 w-3.5" />
        {/if}
        校验
      </Button>
      {#if configEditor.supportsPlanApply}
        <Button
          variant="outline"
          size="sm"
          onclick={handlePlanApply}
          disabled={!canPlanApply || isLoading}
          title="预检配置变更的影响范围（可热加载 vs 需重启）"
        >
          {#if configEditor.phase === 'planning'}
            <Loader2 class="h-3.5 w-3.5 animate-spin" />
          {:else}
            <ScanSearch class="h-3.5 w-3.5" />
          {/if}
          预检
        </Button>
      {/if}
      <Button
        size="sm"
        variant={needsConfirmApply ? 'destructive' : 'default'}
        onclick={handleApply}
        disabled={!canApply || isLoading}
        title={needsConfirmApply
          ? '部分变更需要重启内核才能生效，点击确认应用'
          : '校验并应用到运行中的内核（热加载，无需重启）'}
      >
        {#if configEditor.phase === 'applying'}
          <Loader2 class="h-3.5 w-3.5 animate-spin" />
        {:else if needsConfirmApply}
          <AlertTriangle class="h-3.5 w-3.5" />
        {:else}
          <Send class="h-3.5 w-3.5" />
        {/if}
        {needsConfirmApply ? '确认应用' : '应用'}
      </Button>
    </div>
  </div>

  <!-- Validation errors -->
  {#if hasValidationErrors}
    <div class="error-panel">
      <div class="error-title">
        <AlertTriangle class="h-3.5 w-3.5" />
        <span>校验失败</span>
      </div>
      {#each configEditor.validationErrors as err, i (i)}
        <div class="error-item">
          {#if err.fieldPath}
            <span class="error-field">{err.fieldPath}</span>
          {/if}
          <span class="error-msg">{err.message}</span>
        </div>
      {/each}
    </div>
  {/if}

  <!-- Last error -->
  {#if configEditor.lastError && !hasValidationErrors}
    <div class="error-panel">
      <div class="error-title">
        <AlertTriangle class="h-3.5 w-3.5" />
        <span>{configEditor.lastError}</span>
      </div>
    </div>
  {/if}

  <!-- Plan-apply impact preview -->
  {#if hasPlanResult}
    {@const plan = configEditor.planResult!}
    <div class="plan-panel">
      <div class="plan-header">
        <ScanSearch class="h-3.5 w-3.5" />
        <span>配置变更影响分析</span>
      </div>

      <!-- Hot-reload sections -->
      {#if plan.hotReload.length > 0}
        <div class="plan-section">
          <div class="plan-section-title hot">
            <Zap class="h-3 w-3" />
            <span>可热加载（无需重启）</span>
            <span class="plan-count">{plan.hotReload.length}</span>
          </div>
          {#each plan.hotReload as item (item.section)}
            <div class="plan-item hot">
              <span class="plan-item-section">{sectionLabel(item.section)}</span>
              {#if item.tags.length > 0}
                <span class="plan-item-tags">{item.tags.join(', ')}</span>
              {/if}
              {#if item.detail}
                <span class="plan-item-detail">{item.detail}</span>
              {/if}
            </div>
          {/each}
        </div>
      {/if}

      <!-- Requires-restart sections -->
      {#if plan.requiresRestart.length > 0}
        <div class="plan-section">
          <div class="plan-section-title restart">
            <Power class="h-3 w-3" />
            <span>需要重启内核</span>
            <span class="plan-count restart">{plan.requiresRestart.length}</span>
          </div>
          {#each plan.requiresRestart as item (item.section)}
            <div class="plan-item restart">
              <span class="plan-item-section">{sectionLabel(item.section)}</span>
              {#if item.tags.length > 0}
                <span class="plan-item-tags">{item.tags.join(', ')}</span>
              {/if}
              {#if item.detail}
                <span class="plan-item-detail">{item.detail}</span>
              {/if}
            </div>
          {/each}
        </div>
      {/if}

      <!-- Warnings -->
      {#if plan.warnings.length > 0}
        <div class="plan-warnings">
          {#each plan.warnings as w (w)}
            <div class="plan-warning-item">
              <AlertTriangle class="h-3 w-3" />
              <span>{w}</span>
            </div>
          {/each}
        </div>
      {/if}

      <!-- Empty impact (no changes detected) -->
      {#if plan.hotReload.length === 0 && plan.requiresRestart.length === 0}
        <div class="plan-empty">未检测到配置变更</div>
      {/if}
    </div>
  {/if}

  <!-- Editor -->
  {#if configEditor.phase !== 'idle'}
    <div class="editor-container">
      <div class="line-numbers">
        {#each Array(jsonLineCount()) as _, i (i)}
          <div class="line-number">{i + 1}</div>
        {/each}
      </div>
      <textarea
        bind:this={textareaRef}
        class="editor-textarea"
        spellcheck="false"
        value={configEditor.draftJson}
        oninput={handleInput}
        onkeydown={handleKeyDown}
        disabled={isLoading}
        placeholder={'{}'}
      ></textarea>
    </div>
  {:else}
    <div class="empty-editor">
      <span class="empty-text">点击刷新按钮加载内核当前配置</span>
    </div>
  {/if}
</div>

<style>
  .panel {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 12px;
  }

  .heading {
    display: flex;
    flex-direction: column;
    gap: 4px;
    min-width: 0;
  }

  .title {
    font-size: 13px;
    font-weight: 700;
    color: var(--foreground);
  }

  .desc {
    font-size: 11px;
    color: var(--muted-foreground);
    line-height: 1.5;
  }

  .actions {
    display: flex;
    align-items: center;
    gap: 6px;
    flex-shrink: 0;
  }

  /* Status bar */
  .status-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    padding: 6px 10px;
    border-radius: 6px;
    background: var(--muted);
    border: 1px solid var(--border);
    flex-wrap: wrap;
  }

  .status-left {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-wrap: wrap;
  }

  .status-badge {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    font-size: 11px;
    font-weight: 600;
    padding: 2px 8px;
    border-radius: 4px;
  }

  .status-badge.ready {
    background: rgba(34, 197, 94, 0.1);
    color: #22C55E;
  }

  .status-badge.editing {
    background: rgba(59, 130, 246, 0.1);
    color: #3B82F6;
  }

  .status-badge.loading {
    background: rgba(234, 179, 8, 0.1);
    color: #EAB308;
  }

  .status-badge.success {
    background: rgba(34, 197, 94, 0.1);
    color: #22C55E;
  }

  .status-badge.planned {
    background: rgba(59, 130, 246, 0.1);
    color: #3B82F6;
  }

  .status-badge.error {
    background: rgba(239, 68, 68, 0.1);
    color: #EF4444;
  }

  .dirty-badge {
    font-size: 10px;
    font-weight: 600;
    padding: 1px 6px;
    border-radius: 3px;
    background: rgba(245, 158, 11, 0.1);
    color: #F59E0B;
  }

  .applied-time {
    font-size: 10px;
    color: var(--muted-foreground);
    font-family: var(--font-mono);
  }

  .actions-bar {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  /* Error panel */
  .error-panel {
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 8px 10px;
    border-radius: 6px;
    border: 1px solid rgba(239, 68, 68, 0.2);
    background: rgba(239, 68, 68, 0.06);
  }

  .error-title {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 12px;
    font-weight: 600;
    color: #EF4444;
  }

  .error-item {
    display: flex;
    gap: 6px;
    font-size: 11px;
    color: var(--foreground);
    padding-left: 20px;
  }

  .error-field {
    font-family: var(--font-mono);
    color: #F59E0B;
    flex-shrink: 0;
  }

  .error-msg {
    color: var(--muted-foreground);
  }

  /* Editor */
  .editor-container {
    display: flex;
    border: 1px solid var(--border);
    border-radius: 6px;
    overflow: hidden;
    background: var(--card);
    max-height: 480px;
  }

  .line-numbers {
    display: flex;
    flex-direction: column;
    padding: 8px 6px 8px 8px;
    background: var(--muted);
    border-right: 1px solid var(--border);
    user-select: none;
    overflow: hidden;
    flex-shrink: 0;
  }

  .line-number {
    font-family: var(--font-mono);
    font-size: 11px;
    line-height: 1.5;
    color: var(--muted-foreground);
    text-align: right;
    min-width: 2em;
    opacity: 0.5;
  }

  .editor-textarea {
    flex: 1;
    padding: 8px 10px;
    font-family: var(--font-mono);
    font-size: 12px;
    line-height: 1.5;
    color: var(--foreground);
    background: transparent;
    border: none;
    outline: none;
    resize: none;
    min-height: 200px;
    max-height: 480px;
    tab-size: 2;
    white-space: pre;
    overflow: auto;
  }

  .editor-textarea:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .empty-editor {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 40px;
    border: 1px dashed var(--border);
    border-radius: 6px;
  }

  .empty-text {
    font-size: 12px;
    color: var(--muted-foreground);
  }

  /* Plan-apply impact panel */
  .plan-panel {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 10px 12px;
    border-radius: 6px;
    border: 1px solid var(--border);
    background: var(--card);
  }

  .plan-header {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 12px;
    font-weight: 700;
    color: var(--foreground);
  }

  .plan-section {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .plan-section-title {
    display: flex;
    align-items: center;
    gap: 5px;
    font-size: 11px;
    font-weight: 600;
    padding: 3px 0;
  }

  .plan-section-title.hot {
    color: #22C55E;
  }

  .plan-section-title.restart {
    color: #F59E0B;
  }

  .plan-count {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 18px;
    height: 16px;
    font-size: 10px;
    font-weight: 700;
    padding: 0 4px;
    border-radius: 8px;
    background: rgba(34, 197, 94, 0.15);
    color: #22C55E;
  }

  .plan-count.restart {
    background: rgba(245, 158, 11, 0.15);
    color: #F59E0B;
  }

  .plan-item {
    display: flex;
    align-items: baseline;
    gap: 6px;
    font-size: 11px;
    padding: 3px 0 3px 20px;
    flex-wrap: wrap;
  }

  .plan-item-section {
    font-weight: 600;
    color: var(--foreground);
    flex-shrink: 0;
  }

  .plan-item-tags {
    font-family: var(--font-mono);
    font-size: 10px;
    color: var(--muted-foreground);
    background: var(--muted);
    padding: 1px 4px;
    border-radius: 3px;
  }

  .plan-item-detail {
    color: var(--muted-foreground);
    font-size: 10px;
  }

  .plan-warnings {
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding-top: 4px;
    border-top: 1px solid var(--border);
  }

  .plan-warning-item {
    display: flex;
    align-items: center;
    gap: 5px;
    font-size: 10px;
    color: #F59E0B;
    padding-left: 4px;
  }

  .plan-empty {
    font-size: 11px;
    color: var(--muted-foreground);
    text-align: center;
    padding: 4px 0;
  }

  @media (max-width: 640px) {
    .status-bar {
      flex-direction: column;
      align-items: flex-start;
    }

    .actions-bar {
      width: 100%;
      flex-wrap: wrap;
    }
  }
</style>
