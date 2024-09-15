import { redirect } from '@sveltejs/kit';
import store from '$lib/store';

export async function load({ params }) {
  if (!store.isLoaded) throw redirect(302, '/');
  const scenario = store.findScenario(params.id);
  if (!scenario) throw redirect(302, '/');

  return {
    scenario,
    filePath: scenario.feature?.filePath ?? '',
    scenarioName: scenario.name
  };
}
