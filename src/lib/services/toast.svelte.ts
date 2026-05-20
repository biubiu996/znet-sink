import { SvelteMap } from 'svelte/reactivity';

export type ToastType = 'success' | 'error' | 'warning' | 'info';

export interface Toast {
  id: number;
  type: ToastType;
  message: string;
  duration: number;
}

let nextId = 0;
const toasts = new SvelteMap<number, Toast>();

export function showToast(type: ToastType, message: string, duration: number = 4000): number {
  const id = ++nextId;
  toasts.set(id, { id, type, message, duration });

  if (duration > 0) {
    setTimeout(() => {
      toasts.delete(id);
    }, duration);
  }

  return id;
}

export function dismissToast(id: number): void {
  toasts.delete(id);
}

export function success(message: string, duration?: number): number {
  return showToast('success', message, duration);
}

export function error(message: string, duration?: number): number {
  return showToast('error', message, duration);
}

export function warning(message: string, duration?: number): number {
  return showToast('warning', message, duration);
}

export function info(message: string, duration?: number): number {
  return showToast('info', message, duration);
}

export function getToasts() {
  return toasts;
}
