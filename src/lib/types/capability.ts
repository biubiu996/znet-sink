// Mirror of Rust models::capability

import type { ProxyConfigCapabilities } from './domain';

export interface CapabilityItem {
  key: string;
  enabled: boolean;
  reason?: string;
}

export interface GuiCapabilitySnapshot {
  management: CapabilityItem[];
  proxyFeatures: CapabilityItem[];
  activeProxyConfigId?: string;
  activeProxyConfigCapabilities: ProxyConfigCapabilities;
}
