<script lang="ts">
  import { store } from '$lib/services/store.svelte';
  import { listProxyConfigs, removeProxyConfig, upsertProxyConfig } from '$lib/services/config';
  import { handleAppError } from '$lib/services/core';
  import type { ProxyConfigProfile, ProxyConfigUpsert } from '$lib/types/domain';

  let configs = $state<ProxyConfigProfile[]>([]);
  let loading = $state(true);
  let showForm = $state(false);
  let saving = $state(false);
  let editingId = $state<string | null>(null);

  let form = $state({ name: '', format: 'json', content: '', active: false });

  async function refresh() {
    loading = true;
    try {
      configs = await listProxyConfigs();
    } catch (e) {
      console.error('Failed to load proxy configs:', e);
    } finally {
      loading = false;
    }
  }

  async function handleRemove(id: string) {
    if (!confirm('确认删除此配置？')) return;
    try {
      await removeProxyConfig(id);
      await refresh();
    } catch (e) {
      handleAppError(e, '删除代理配置失败');
    }
  }

  function openCreate() {
    editingId = null;
    form = { name: '', format: 'json', content: '', active: false };
    showForm = true;
  }

  function openEdit(config: ProxyConfigProfile) {
    editingId = config.id;
    form = {
      name: config.name,
      format: config.format,
      content: config.content ? JSON.stringify(config.content, null, 2) : '',
      active: config.active,
    };
    showForm = true;
  }

  async function handleSave() {
    if (!form.name.trim()) return;
    saving = true;
    try {
      let content: unknown = undefined;
      if (form.content.trim()) {
        try {
          content = JSON.parse(form.content);
        } catch {
          alert('代理配置内容不是有效的 JSON');
          saving = false;
          return;
        }
      }

      const input: ProxyConfigUpsert = {
        id: editingId ?? undefined,
        name: form.name.trim(),
        format: form.format || undefined,
        content: content ?? undefined,
        active: form.active || undefined,
      };

      await upsertProxyConfig(input);
      showForm = false;
      await refresh();
    } catch (e) {
      handleAppError(e, '保存代理配置失败');
    } finally {
      saving = false;
    }
  }

  $effect(() => {
    refresh();
  });
</script>

<div class="desk-card flex-1 overflow-hidden flex flex-col animate-fade-in">
  <!-- Panel header -->
  <div class="panel-header">
    <span class="panel-title">代理配置</span>
    {#if store.isActionOperable('proxyConfig.upsert')}
      <button class="action-btn" onclick={openCreate}>
        <svg width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round">
          <line x1="6" y1="1" x2="6" y2="11"/><line x1="1" y1="6" x2="11" y2="6"/>
        </svg>
        新增
      </button>
    {/if}
  </div>

  <!-- Content -->
  {#if loading}
    <div class="panel-empty">加载中...</div>
  {:else if configs.length === 0 && !showForm}
    <div class="panel-empty">暂无配置，点击新增导入或创建</div>
  {:else}
    <div class="list-scroll">
      {#each configs as config (config.id)}
        <div
          role="button"
          tabindex="0"
          onclick={() => openEdit(config)}
          onkeydown={(e) => e.key === 'Enter' && openEdit(config)}
          class="list-row"
        >
          <div class="row-main">
            <div class="row-top">
              <span class="row-name">{config.name}</span>
              <span class="row-tag">{config.format}</span>
              {#if config.active}
                <span class="row-tag active-tag">活跃</span>
              {/if}
            </div>
            <span class="row-sub font-mono">{config.id}</span>
          </div>
          {#if store.isActionOperable('proxyConfig.remove')}
            <button
              class="row-del"
              onclick={(e: MouseEvent) => { e.stopPropagation(); handleRemove(config.id); }}
              title="删除"
            >
              <svg width="14" height="14" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round">
                <line x1="2" y1="2" x2="10" y2="10"/><line x1="10" y1="2" x2="2" y2="10"/>
              </svg>
            </button>
          {/if}
        </div>
      {/each}
    </div>
  {/if}
</div>

<!-- Modal -->
{#if showForm}
  <div
    class="modal-backdrop"
    onclick={() => showForm = false}
    onkeydown={(e) => e.key === 'Escape' && (showForm = false)}
    role="button"
    tabindex="0"
    aria-label="关闭弹窗"
  >
    <div
      class="modal-box"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
      role="dialog"
      aria-modal="true"
      tabindex="-1"
    >
      <h4 class="modal-title">{editingId ? '编辑' : '新增'}代理配置</h4>

      <div class="modal-fields">
        <div class="field-row">
          <label for="profile-name" class="field-label">名称 <span class="required">*</span></label>
          <input
            id="profile-name"
            bind:value={form.name}
            placeholder="例如: 香港节点配置"
            class="field-input"
          />
        </div>

        <div class="field-row">
          <label for="profile-format" class="field-label">格式</label>
          <select id="profile-format" bind:value={form.format} class="field-input">
            <option value="json">JSON (标准)</option>
            <option value="zero">Zero 内核格式</option>
          </select>
        </div>

        <div class="field-row">
          <label for="profile-content" class="field-label">JSON 内容</label>
          <textarea
            id="profile-content"
            bind:value={form.content}
            placeholder='粘贴代理配置 JSON...'
            rows={8}
            class="field-input field-mono resize-y"
          ></textarea>
        </div>

        <div class="field-row field-toggle">
          <span class="field-label">设为活跃配置</span>
          <button
            onclick={() => form.active = !form.active}
            class="toggle-btn {form.active ? 'on' : ''}"
            role="switch"
            aria-checked={form.active}
          >
            <span class="toggle-thumb"></span>
          </button>
        </div>
      </div>

      <div class="modal-actions">
        <button class="btn-ghost" onclick={() => showForm = false}>取消</button>
        <button class="btn-primary" onclick={handleSave} disabled={saving || !form.name.trim()}>
          {saving ? '保存中...' : '保存'}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  /* Panel structure */
  .panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 11px 14px 10px;
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
  }

  .panel-title {
    font-size: 13px;
    font-weight: 600;
    color: var(--foreground);
    letter-spacing: -0.01em;
  }

  .panel-empty {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 12px;
    color: var(--muted-foreground);
  }

  .action-btn {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    padding: 5px 10px;
    border-radius: 7px;
    font-size: 12px;
    font-weight: 500;
    background: var(--muted);
    color: var(--foreground);
    border: 1px solid var(--border);
    cursor: pointer;
    transition: background 0.12s ease;
  }

  .action-btn:hover {
    background: var(--surface);
  }

  /* List */
  .list-scroll {
    flex: 1;
    overflow-y: auto;
    padding: 5px;
    display: flex;
    flex-direction: column;
    gap: 1px;
    min-height: 0;
  }

  .list-row {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 9px 11px;
    border-radius: 8px;
    border: 1px solid transparent;
    cursor: pointer;
    transition: background 0.12s ease, border-color 0.12s ease;
  }

  .list-row:hover {
    background: var(--muted);
    border-color: var(--border);
  }

  .row-main {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 3px;
  }

  .row-top {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .row-name {
    font-size: 12px;
    font-weight: 500;
    color: var(--foreground);
  }

  .row-tag {
    font-size: 10px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    padding: 2px 6px;
    border-radius: 4px;
    background: var(--muted);
    color: var(--muted-foreground);
  }

  .row-tag.active-tag {
    background: rgba(34, 197, 94, 0.12);
    color: var(--success);
  }

  .row-sub {
    font-size: 10px;
    color: var(--muted-foreground);
    opacity: 0.55;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .row-del {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    border-radius: 6px;
    background: transparent;
    color: var(--muted-foreground);
    border: none;
    cursor: pointer;
    opacity: 0;
    transition: opacity 0.12s ease, background 0.12s ease, color 0.12s ease;
    flex-shrink: 0;
  }

  .list-row:hover .row-del {
    opacity: 1;
  }

  .row-del:hover {
    background: rgba(239, 68, 68, 0.1);
    color: var(--destructive);
  }

  /* Modal */
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.4);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 50;
    backdrop-filter: blur(4px);
  }

  :global(.dark) .modal-backdrop {
    background: rgba(0, 0, 0, 0.6);
  }

  .modal-box {
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 14px;
    padding: 18px;
    width: 400px;
    max-height: 80vh;
    overflow-y: auto;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.15);
  }

  :global(.dark) .modal-box {
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
  }

  .modal-title {
    font-size: 14px;
    font-weight: 600;
    color: var(--foreground);
    margin-bottom: 16px;
  }

  .modal-fields {
    display: flex;
    flex-direction: column;
    gap: 11px;
  }

  .field-row {
    display: flex;
    flex-direction: column;
    gap: 5px;
  }

  .field-toggle {
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    padding: 10px 0 5px;
    border-top: 1px solid var(--border);
  }

  .field-label {
    font-size: 11px;
    font-weight: 500;
    color: var(--muted-foreground);
  }

  .required {
    color: var(--destructive);
  }

  .field-input {
    width: 100%;
    padding: 7px 10px;
    border-radius: 7px;
    background: var(--muted);
    border: 1px solid var(--border);
    color: var(--foreground);
    font-size: 12px;
    outline: none;
    transition: border-color 0.12s ease;
  }

  .field-input:focus {
    border-color: rgba(99, 102, 241, 0.4);
  }

  .field-mono {
    font-family: var(--font-mono);
  }

  /* Toggle switch */
  .toggle-btn {
    position: relative;
    width: 34px;
    height: 20px;
    border-radius: 10px;
    background: var(--muted);
    border: 1px solid var(--border);
    cursor: pointer;
    transition: background 0.15s ease, border-color 0.15s ease;
    flex-shrink: 0;
  }

  .toggle-btn.on {
    background: var(--primary);
    border-color: transparent;
  }

  .toggle-thumb {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: var(--muted-foreground);
    transition: transform 0.15s ease, background 0.15s ease;
  }

  .toggle-btn.on .toggle-thumb {
    transform: translateX(14px);
    background: var(--primary-foreground);
  }

  /* Action buttons */
  .modal-actions {
    display: flex;
    gap: 8px;
    margin-top: 16px;
  }

  .btn-ghost {
    flex: 1;
    padding: 8px 14px;
    border-radius: 8px;
    background: var(--muted);
    color: var(--muted-foreground);
    font-size: 12px;
    font-weight: 500;
    border: 1px solid var(--border);
    cursor: pointer;
    transition: background 0.12s ease, color 0.12s ease;
  }

  .btn-ghost:hover {
    background: var(--surface);
    color: var(--foreground);
  }

  .btn-primary {
    flex: 1;
    padding: 8px 14px;
    border-radius: 8px;
    background: var(--primary);
    color: var(--primary-foreground);
    font-size: 12px;
    font-weight: 500;
    border: none;
    cursor: pointer;
    transition: opacity 0.12s ease;
  }

  .btn-primary:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .btn-primary:not(:disabled):hover {
    opacity: 0.88;
  }
</style>
