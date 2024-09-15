import { redirect } from '@sveltejs/kit';
import store from '$lib/store';

export async function load({ params }) {
  if (!store.isLoaded) throw redirect(302, '/');
  const organization = store.findOrganization(params.id);
  if (!organization) throw redirect(302, '/');

  return {
    organization,
  };
}
