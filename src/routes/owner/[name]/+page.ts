import { redirect } from '@sveltejs/kit';
import store from '$lib/store';

export async function load({ params }) {
  if (!store.isLoaded) throw redirect(302, '/');
  const owner = store.findOwner(params.name);
  if (!owner) throw redirect(302, '/');

  return {
    owner,
  };
}
