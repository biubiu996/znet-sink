import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { startCoreEvents, stopCoreEvents, appendLog, getCoreStats, getCoreRuntime } from '$lib/services/core';
import { overviewData } from '$lib/services/overview-data.svelte';
import type { CoreEventPayload, CoreEventStatus } from '$lib/types/core';

const EVENT_NAME = 'core:event';
const STATUS_NAME = 'core:event-status';

class CoreEventsService {
  isSubscribed = $state(false);
  status = $state<'idle' | 'subscribed' | 'offline' | 'error' | 'disconnected'>('idle');
  lastError = $state<string | null>(null);

  private _unlistenEvent: UnlistenFn | null = null;
  private _unlistenStatus: UnlistenFn | null = null;
  private _activeGeneration: number | null = null;

  async start(events?: string[]) {
    // Listen before starting subscription so we don't miss status events
    if (!this._unlistenEvent) {
      this._unlistenEvent = await listen<CoreEventPayload>(EVENT_NAME, (event) => {
        this._routeEvent(event.payload);
      });
    }
    if (!this._unlistenStatus) {
      this._unlistenStatus = await listen<CoreEventStatus>(STATUS_NAME, (event) => {
        this._handleStatus(event.payload);
      });
    }

    try {
      const sub = await startCoreEvents(events);
      this._activeGeneration = sub.generation;
    } catch (e) {
      this.status = 'error';
      this.lastError = String(e);
    }
  }

  stop() {
    stopCoreEvents();
    this._activeGeneration = null;
    this.isSubscribed = false;
    this.status = 'idle';
    this._unlistenEvent?.();
    this._unlistenStatus?.();
    this._unlistenEvent = null;
    this._unlistenStatus = null;
  }

  private _handleStatus(status: CoreEventStatus) {
    if (status.generation !== this._activeGeneration) return;

    switch (status.status) {
      case 'subscribed':
        this.isSubscribed = true;
        this.status = 'subscribed';
        this.lastError = null;
        // Fetch initial state snapshot to fill gaps
        this._fetchInitialState();
        break;
      case 'offline':
        this.isSubscribed = false;
        this.status = 'offline';
        this.lastError = status.error?.message ?? 'core is not available';
        overviewData.isLive = false;
        break;
      case 'disconnected':
        this.isSubscribed = false;
        this.status = 'disconnected';
        overviewData.isLive = false;
        break;
      case 'stopped':
        this.isSubscribed = false;
        this.status = 'idle';
        break;
      case 'error':
        this.isSubscribed = false;
        this.status = 'error';
        this.lastError = status.error?.message ?? 'unknown error';
        overviewData.isLive = false;
        break;
    }
  }

  private _routeEvent(payload: CoreEventPayload) {
    const { generation: _gen, event } = payload;
    if (!event || typeof event !== 'object') return;

    const obj = event as Record<string, unknown>;
    const type = typeof obj['type'] === 'string' ? obj['type'] : '';
    const subtype = typeof obj['subtype'] === 'string' ? obj['subtype'] : '';

    // Stats events
    if (
      type === 'stats' ||
      subtype === 'stats' ||
      this._hasAnyKey(obj, ['uploadSpeed', 'downloadSpeed', 'upload_speed', 'download_speed', 'txSpeed', 'rxSpeed', 'connections', 'connectionCount'])
    ) {
      overviewData.applyStatsEvent(obj);
      return;
    }

    // Runtime / node events
    if (
      type === 'runtime' ||
      type === 'config' ||
      this._hasAnyKey(obj, ['proxies', 'outbounds', 'nodes'])
    ) {
      overviewData.applyRuntimeEvent(obj);
      return;
    }

    // Log events
    if (
      type === 'log' ||
      subtype === 'log' ||
      (typeof obj['level'] === 'string' && typeof obj['message'] === 'string')
    ) {
      const level = (typeof obj['level'] === 'string' ? obj['level'] : 'info') as 'trace' | 'debug' | 'info' | 'warn' | 'error';
      const message = typeof obj['message'] === 'string' ? obj['message'] : JSON.stringify(obj);
      appendLog({ source: 'core', level, message, fields: obj }).catch(() => {});
      return;
    }

    // Connection / flow events
    if (
      type === 'flow' ||
      type === 'connection' ||
      typeof obj['flow_id'] === 'string' ||
      typeof obj['flowId'] === 'string'
    ) {
      // Future: route to connection store
      return;
    }
  }

  private _hasAnyKey(obj: Record<string, unknown>, keys: string[]): boolean {
    return keys.some((k) => k in obj);
  }

  private async _fetchInitialState() {
    try {
      const [statsResult, runtimeResult] = await Promise.all([
        getCoreStats(),
        getCoreRuntime(),
      ]);
      if (statsResult.available && statsResult.response) {
        overviewData.applyStatsEvent(statsResult.response as Record<string, unknown>);
      }
      if (runtimeResult.available && runtimeResult.response) {
        overviewData.applyRuntimeEvent(runtimeResult.response as Record<string, unknown>);
      }
    } catch {
      // Best-effort initial fetch
    }
  }
}

export const coreEvents = new CoreEventsService();
