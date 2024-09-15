import { writable } from "svelte/store";

export enum ToastType {
  INFO = "info",
  SUCCESS = "success",
  ERROR = "error",
}

type Toast = {
  id: number;
  type: ToastType;
  message: string;
  dismissible: boolean;
  timeout: number;
};

type ToastInput = {
  type?: ToastType;
  message: string;
  dismissible?: boolean;
  timeout?: number;
};

export const toasts = writable<Toast[]>([]);

export const addToast = (input: ToastInput) => {
  // Create a unique ID so we can easily find/remove it
  // if it is dismissible/has a timeout.
  const id = Math.floor(Math.random() * 10000);

  const toast = {
    id,
    type: ToastType.INFO,
    dismissible: true,
    timeout: 3000,
    ...input,
  };

  toasts.update((all) => [{ ...toast }, ...all]);

  if (toast.timeout) setTimeout(() => dismissToast(id), toast.timeout);
};

export const dismissToast = (id: number) => {
  toasts.update((all) => all.filter((t) => t.id !== id));
};
