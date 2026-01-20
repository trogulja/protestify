import { LazyStore } from "@tauri-apps/plugin-store";
import { writable } from "svelte/store";

const store = new LazyStore('.settings.dat');

export type Theme = 'system' | 'light' | 'dark';

export interface SettingsValue {
  basePath: string;
  e2ePwd: string;
  e2eUrl: string;
  codeEditor: string;
  backofficeApiKey: string;
  theme: Theme;
}

function settingsStore() {
  const initialValue: SettingsValue = {
    basePath: '',
    e2ePwd: '',
    e2eUrl: '',
    codeEditor: '',
    backofficeApiKey: '',
    theme: 'system',
  }

  let isInitialized = false;

  const { subscribe, set } = writable(initialValue);

  function update(values: SettingsValue) {
    isInitialized = false;
    const keys = Object.keys(values) as Array<keyof SettingsValue>;
    Promise
      .all(keys.map((key) => store.set(key, values[key])))
      .then(() => {
        set(values);
        isInitialized = true;
      });
  }

  async function init() {
    if (isInitialized) return;

    const updated = { ...initialValue };
    for (const key of Object.keys(initialValue) as Array<keyof SettingsValue>) {
      const defaultVal = initialValue[key];
      const value = await store.get<typeof defaultVal>(key);
      if (value !== null && value !== undefined) {
        (updated as Record<string, unknown>)[key] = value;
      }
    }

    set(updated);
  }

	return { subscribe, update, init }
}

export default settingsStore();
