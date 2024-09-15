import { redirect } from '@sveltejs/kit';
import { check } from '@tauri-apps/plugin-updater';
import store from '$lib/store';
import isUpdateAvailable from '$lib/store/is-update-available';

export async function load() {
  // Try to load the data, but only once (it's expensive)
  if (!store.isLoaded) await store.loadData();

  // If the data is still not loaded, redirect to the settings page
  if (!store.isLoaded) throw redirect(302, '/settings');

  // Check if an update is available
  try {
    const update = await check();
    if (update?.available) isUpdateAvailable.update((_) => true);
  } catch (e) {
    console.error('Failed to check for updates', e);
  }

  return {
    tableData: store.tableData.scenarios,
  };
}
