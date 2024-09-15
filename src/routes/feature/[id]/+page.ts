import { redirect } from '@sveltejs/kit';
import store from '$lib/store';

export async function load({ params }) {
  if (!store.isLoaded) throw redirect(302, '/');
  const feature = store.findFeature(params.id);
  if (!feature) throw redirect(302, '/');

  return {
    feature,
    filePath: feature.filePath ?? '',
    scenarios: feature.scenarios ?? [],
  };
}
