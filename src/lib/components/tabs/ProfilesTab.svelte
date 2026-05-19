<script lang="ts">
  import { listProxyConfigs, removeProxyConfig, upsertProxyConfig, importProxyConfig } from '$lib/services/config';
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
    await removeProxyConfig(id);
    await refresh();
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
      console.error('Failed to save proxy config:', e);
      alert(`保存失败: ${e}`);
    } finally {
      saving = false;
    }
  }

  $effect(() => {
    refresh();
  });
</script>

<div class="flex-1 w-full bg-card border border-card-border rounded-xl p-4 flex flex-col gap-4 animate-fade-in overflow-hidden">
  <div class="flex items-center justify-between flex-shrink-0">
    <h3 class="text-sm font-bold text-foreground">代理配置</h3>
    <button
      onclick={openCreate}
      class="px-3 py-1.5 rounded-lg bg-primary text-primary-foreground text-xs font-medium"
    >
      + 新增
    </button>
  </div>

  {#if loading}
    <div class="flex-1 flex items-center justify-center text-xs text-muted-foreground">加载中...</div>
  {:else if configs.length === 0 && !showForm}
    <div class="flex-1 flex items-center justify-center text-xs text-muted-foreground">暂无配置，点击新增导入或创建</div>
  {:else}
    <div class="flex-1 overflow-y-auto min-h-0">
      <div class="grid grid-cols-1 gap-2">
        {#each configs as config (config.id)}
          <div
            role="button"
            tabindex="0"
            onclick={() => openEdit(config)}
            onkeydown={(e) => e.key === 'Enter' && openEdit(config)}
            class="bg-muted/30 border border-card-border rounded-lg p-3 flex items-center justify-between text-left hover:bg-muted/50 transition-colors cursor-pointer"
          >
            <div class="flex flex-col gap-1">
              <div class="flex items-center gap-2">
                <span class="text-xs font-medium text-foreground">{config.name}</span>
                <span class="text-[10px] px-1.5 py-0.5 rounded bg-muted text-muted-foreground">{config.format}</span>
                {#if config.active}
                  <span class="text-[10px] px-1.5 py-0.5 rounded bg-green-500/20 text-green-500">活跃</span>
                {/if}
              </div>
              <span class="text-[10px] text-muted-foreground font-mono">{config.id}</span>
            </div>
            <button
              onclick={(e: MouseEvent) => { e.stopPropagation(); handleRemove(config.id); }}
              class="text-[10px] px-2 py-1 rounded text-red-500 hover:bg-red-500/10"
            >
              删除
            </button>
          </div>
        {/each}
      </div>
    </div>
  {/if}
</div>

{#if showForm}
  <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" onclick={() => showForm = false} role="dialog">
    <div class="bg-card border border-card-border rounded-xl p-5 w-[420px] max-h-[80vh] overflow-y-auto" onclick={(e) => e.stopPropagation()}>
      <h4 class="text-sm font-bold text-foreground mb-4">{editingId ? '编辑' : '新增'}代理配置</h4>

      <div class="space-y-3">
        <div>
          <label class="text-[10px] text-muted-foreground block mb-1">名称 *</label>
          <input
            bind:value={form.name}
            placeholder="例如: 香港节点配置"
            class="w-full px-3 py-2 rounded-lg bg-muted text-xs text-foreground border border-card-border focus:border-primary outline-none"
          />
        </div>

        <div>
          <label class="text-[10px] text-muted-foreground block mb-1">格式</label>
          <select
            bind:value={form.format}
            class="w-full px-3 py-2 rounded-lg bg-muted text-xs text-foreground border border-card-border outline-none"
          >
            <option value="json">JSON (标准)</option>
            <option value="zero">Zero 内核格式</option>
          </select>
        </div>

        <div>
          <label class="text-[10px] text-muted-foreground block mb-1">JSON 内容</label>
          <textarea
            bind:value={form.content}
            placeholder='粘贴代理配置 JSON...'
            rows={10}
            class="w-full px-3 py-2 rounded-lg bg-muted text-xs text-foreground border border-card-border focus:border-primary outline-none font-mono resize-y"
          ></textarea>
        </div>

        <div class="flex items-center justify-between">
          <span class="text-[10px] text-muted-foreground">设为活跃配置</span>
          <button
            onclick={() => form.active = !form.active}
            class="w-9 h-5 rounded-full relative transition-colors {form.active ? 'bg-primary' : 'bg-muted'}"
          >
            <div class="w-4 h-4 rounded-full bg-white absolute top-0.5 transition-all shadow {form.active ? 'left-4' : 'left-0.5'}"></div>
          </button>
        </div>
      </div>

      <div class="flex gap-2 mt-5">
        <button
          onclick={() => showForm = false}
          class="flex-1 py-2 rounded-lg bg-muted text-muted-foreground text-xs font-medium"
        >
          取消
        </button>
        <button
          onclick={handleSave}
          disabled={saving || !form.name.trim()}
          class="flex-1 py-2 rounded-lg bg-primary text-primary-foreground text-xs font-medium disabled:opacity-50"
        >
          {saving ? '保存中...' : '保存'}
        </button>
      </div>
    </div>
  </div>
{/if}
