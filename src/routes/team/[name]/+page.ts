import { redirect } from '@sveltejs/kit';
import store from '$lib/store';

export async function load({ params }) {
  if (!store.isLoaded) throw redirect(302, '/');
  const team = store.findTeam(params.name);
  if (!team) throw redirect(302, '/');

  return {
    team,
  };
}
