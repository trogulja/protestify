import { Store } from "@tauri-apps/plugin-store";
import { writable } from "svelte/store";

const store = new Store('.settings.dat');

function settingsStore() {
  const initialValue = {
    basePath: '',
    e2ePwd: '',
    e2eUrl: '',
    codeEditor: '',
  }

  let isInitialized = false;

  const { subscribe, set } = writable(initialValue);

  function update(values: typeof initialValue) {
    isInitialized = false;
    const keys = Object.keys(values) as Array<keyof typeof values>;
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
    for (const key in updated) {
      const value = await store.get<string>(key);
      updated[key as keyof typeof updated] = value ?? '';
    }

    set(updated);
  }

	return { subscribe, update, init }
}

export default settingsStore();
