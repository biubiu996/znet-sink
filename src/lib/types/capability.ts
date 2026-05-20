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

export interface InteractionSurfaceItem {
  key: string;
  category: string;
  visible: boolean;
  operable: boolean;
  readonly: boolean;
  reason?: string;
}

export interface InteractionSurfaceSnapshot {
  uiMode: string;
  navigation: InteractionSurfaceItem[];
  actions: InteractionSurfaceItem[];
  features: InteractionSurfaceItem[];
}
