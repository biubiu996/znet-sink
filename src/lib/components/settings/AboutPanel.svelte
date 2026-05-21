<script lang="ts">
  import { getName, getVersion, getTauriVersion } from '@tauri-apps/api/app';

  let appName = $state('ZNet Sink');
  let appVersion = $state('0.0.1');
  let tauriVersion = $state('');
  let loaded = $state(false);

  $effect(() => {
    loadAppInfo();
  });

  async function loadAppInfo() {
    try {
      const [name, ver, tauriVer] = await Promise.all([
        getName(),
        getVersion(),
        getTauriVersion(),
      ]);
      appName = name;
      appVersion = ver;
      tauriVersion = tauriVer;
    } catch {
      // running outside Tauri (browser dev), use defaults from package.json
      appName = 'ZNet Sink';
      appVersion = '0.0.1';
      tauriVersion = '—';
    }
    loaded = true;
  }
</script>

<div class="about-root desk-card">
  <!-- Hero -->
  <div class="about-hero">
    <div class="about-logo">
      <svg width="36" height="36" viewBox="0 0 36 36" fill="none">
        <rect width="36" height="36" rx="10" fill="var(--primary)" opacity="0.12"/>
        <rect x="2" y="2" width="32" height="32" rx="9" stroke="var(--primary)" stroke-width="1.5" opacity="0.3"/>
        <path d="M10 14 L18 10 L26 14" stroke="var(--primary)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        <path d="M10 18 L18 22 L26 18" stroke="var(--primary)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" opacity="0.6"/>
        <path d="M10 22 L18 26 L26 22" stroke="var(--primary)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" opacity="0.35"/>
      </svg>
    </div>
    <div class="about-hero-text">
      <span class="about-hero-name">{appName}</span>
      <span class="about-hero-tagline">零域网络代理客户端</span>
    </div>
    {#if loaded}
      <span class="about-version-badge">v{appVersion}</span>
    {:else}
      <span class="about-version-badge loading">...</span>
    {/if}
  </div>

  <div class="config-separator"></div>

  <!-- App info -->
  <div class="config-section">
    <div class="config-section-title">应用信息</div>

    <div class="config-row">
      <span class="config-label">应用名称</span>
      <span class="config-value">{appName}</span>
    </div>

    <div class="config-row">
      <span class="config-label">版本号</span>
      <span class="config-value mono">v{appVersion}</span>
    </div>

    <div class="config-row">
      <span class="config-label">Tauri 版本</span>
      <span class="config-value mono">{tauriVersion || '—'}</span>
    </div>

    <div class="config-row">
      <span class="config-label">构建标识</span>
      <span class="config-value mono">org.zerdenet.znetsink</span>
    </div>
  </div>

  <div class="config-separator"></div>

  <!-- Resources -->
  <div class="config-section">
    <div class="config-section-title">资源</div>

    <div class="config-row">
      <span class="config-label">仓库地址</span>
      <a
        class="config-value mono link"
        href="https://github.com/zerdenet/znet-sink"
        target="_blank"
        rel="noopener noreferrer"
      >
        github.com/zerdenet/znet-sink
        <svg width="10" height="10" viewBox="0 0 10 10" fill="none" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round" class="link-icon">
          <path d="M3 1H1v8h8V7"/>
          <path d="M6 1h3v3"/>
          <path d="M10 0L4.5 5.5"/>
        </svg>
      </a>
    </div>

    <div class="config-row">
      <span class="config-label">许可证</span>
      <span class="config-value">MIT</span>
    </div>

    <div class="config-row">
      <span class="config-label">描述</span>
      <span class="config-value desc">ZNet Sink 是一款轻量级网络代理管理客户端，基于 Tauri 2 构建，提供配置管理、订阅同步、规则集编辑与实时连接监控等能力。</span>
    </div>
  </div>

  <!-- Footer -->
  <div class="about-footer">
    <span class="about-copyright">&copy; {new Date().getFullYear()} ZeroNet. All rights reserved.</span>
  </div>
</div>

<style>
  .about-root {
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  /* ---- Hero ---- */
  .about-hero {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 12px 12px 14px;
  }

  .about-logo {
    flex-shrink: 0;
  }

  .about-hero-text {
    display: flex;
    flex-direction: column;
    gap: 1px;
    flex: 1;
    min-width: 0;
  }

  .about-hero-name {
    font-size: 15px;
    font-weight: 700;
    letter-spacing: -0.02em;
    color: var(--foreground);
    line-height: 1.3;
  }

  .about-hero-tagline {
    font-size: 11px;
    color: var(--muted-foreground);
    opacity: 0.75;
  }

  .about-version-badge {
    flex-shrink: 0;
    display: inline-flex;
    align-items: center;
    padding: 3px 8px;
    border-radius: 5px;
    font-size: 11px;
    font-weight: 600;
    font-family: var(--font-mono);
    background: var(--muted);
    color: var(--muted-foreground);
  }

  .about-version-badge.loading {
    opacity: 0.4;
  }

  /* ---- Shared: config helpers ---- */
  .config-separator {
    height: 1px;
    background: var(--border);
    margin: 0 12px;
  }

  .config-section {
    display: flex;
    flex-direction: column;
    padding: 8px 0;
  }

  .config-section-title {
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.07em;
    text-transform: uppercase;
    color: var(--muted-foreground);
    padding: 0 12px 6px;
    opacity: 0.7;
  }

  .config-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 6px 12px;
    gap: 12px;
    transition: background 0.1s ease;
  }

  .config-row:hover {
    background: var(--muted);
  }

  .config-label {
    font-size: 12px;
    color: var(--muted-foreground);
    flex-shrink: 0;
    min-width: 72px;
  }

  .config-value {
    font-size: 12px;
    color: var(--foreground);
    text-align: right;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 220px;
  }

  .config-value.mono {
    font-family: var(--font-mono);
    font-size: 12px;
    color: var(--muted-foreground);
  }

  .config-value.desc {
    white-space: normal;
    text-align: right;
    line-height: 1.45;
    font-size: 11.5px;
    max-width: 240px;
    color: var(--muted-foreground);
  }

  .config-value.link {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    color: var(--primary);
    text-decoration: none;
    cursor: pointer;
    transition: opacity 0.12s ease;
  }

  .config-value.link:hover {
    opacity: 0.8;
  }

  .link-icon {
    flex-shrink: 0;
    opacity: 0.5;
  }

  /* ---- Footer ---- */
  .about-footer {
    margin-top: auto;
    padding: 10px 12px;
    border-top: 1px solid var(--border);
    text-align: center;
  }

  .about-copyright {
    font-size: 11px;
    color: var(--muted-foreground);
    opacity: 0.5;
  }
</style>
