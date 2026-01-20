import { redirect } from '@sveltejs/kit';
import store from '$lib/store';

export async function load() {
  if (!store.isLoaded) await store.loadData();
  if (!store.isLoaded) throw redirect(302, '/settings');

  return {
    tableData: store.tableData.scenarios,
  };
}
