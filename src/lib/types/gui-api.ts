// GUI 层业务接口类型定义
// 对应 Rust 后端的 gui_* 命令 DTO

export interface SelfTestCheckItem {
  name: string;
  passed: boolean;
  message?: string;
}

export interface SelfTestSnapshot {
  ready: boolean;
  blockingIssues: string[];
  warningCount: number;
  checks: SelfTestCheckItem[];
  suggestedFlow: 'setup' | 'ready' | 'troubleshoot';
}

export type ConnectionState = 'disconnected' | 'connecting' | 'connected' | 'error';
export type ProxyMode = 'global' | 'rule' | 'direct';

export interface ConnectionStatus {
  state: ConnectionState;
  message?: string;
  uptimeMs?: number;
  activeConnections?: number;
  systemProxyEnabled?: boolean;
}

export interface ProxyModeStatus {
  currentMode: ProxyMode;
  availableModes: ProxyMode[];
  message?: string;
}

export interface CoreOverview {
  coreState: 'stopped' | 'starting' | 'running' | 'stopping' | 'error';
  version?: string;
  uptimeMs?: number;
  memoryUsageBytes?: number;
  cpuUsagePercent?: number;
}

export interface TrafficStats {
  uploadBytesPerSec: number;
  downloadBytesPerSec: number;
  totalUploadBytes: number;
  totalDownloadBytes: number;
  connectionCount: number;
}

export interface PolicyOutbound {
  tag: string;
  type: string;
}

export interface PolicyGroup {
  name: string;
  selected?: string;
  outbounds: PolicyOutbound[];
}
