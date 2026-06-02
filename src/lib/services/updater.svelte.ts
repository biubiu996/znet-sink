import { check } from '@tauri-apps/plugin-updater';
import { info, warning } from '$lib/services/toast.svelte';

class UpdaterService {
  updateAvailable = $state(false);
  latestVersion = $state<string | null>(null);
  currentVersion = $state<string>('0.0.1');
  releaseNotes = $state<string | null>(null);
  checking = $state(false);
  downloading = $state(false);

  /** Check for updates — call on startup and manually from About panel. */
  async checkForUpdate(): Promise<boolean> {
    if (this.checking) return false;
    this.checking = true;
    try {
      const update = await check();
      if (update) {
        this.updateAvailable = true;
        this.latestVersion = update.version;
        this.currentVersion = update.currentVersion;
        this.releaseNotes = update.body ?? null;
        return true;
      } else {
        this.updateAvailable = false;
        this.latestVersion = null;
        return false;
      }
    } catch (e) {
      console.debug('[ZNet] update check failed (may be offline):', e);
      return false;
    } finally {
      this.checking = false;
    }
  }

  /** Download and install the update. */
  async downloadAndInstall(): Promise<boolean> {
    if (this.downloading) return false;
    this.downloading = true;
    try {
      const update = await check();
      if (!update) {
        this.downloading = false;
        return false;
      }

      let downloaded = 0;
      let total: number | undefined;

      await update.downloadAndInstall((event) => {
        switch (event.event) {
          case 'Started':
            total = event.data.contentLength ?? undefined;
            info('开始下载更新…');
            break;
          case 'Progress':
            downloaded += event.data.chunkLength;
            break;
          case 'Finished':
            info('下载完成，应用即将重启…');
            break;
        }
      });

      // The app will restart after install
      this.downloading = false;
      return true;
    } catch (e) {
      warning(`更新失败: ${e instanceof Error ? e.message : String(e)}`);
      this.downloading = false;
      return false;
    }
  }

  /** Manually dismiss the update notification. */
  dismissUpdate() {
    this.updateAvailable = false;
    this.latestVersion = null;
    this.releaseNotes = null;
  }
}

export const updater = new UpdaterService();
