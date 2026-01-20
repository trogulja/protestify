import { redirect } from '@sveltejs/kit';
import store from '$lib/store';
import {
  getStats,
  getBrokenScenarios,
  getOrgsWithoutTests,
  getTeamBreakdown,
  getOwnerBreakdown
} from '$lib/store/dashboard.svelte';

export async function load() {
  if (!store.isLoaded) await store.loadData();
  if (!store.isLoaded) throw redirect(302, '/settings');

  return {
    stats: getStats(),
    brokenScenarios: getBrokenScenarios(),
    orgsWithoutTests: getOrgsWithoutTests(),
    teamBreakdown: getTeamBreakdown(),
    ownerBreakdown: getOwnerBreakdown(),
  };
}
