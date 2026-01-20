import { redirect } from '@sveltejs/kit';
import { invoke } from '@tauri-apps/api/core';
import store from '$lib/store';
import { isInvokeErr } from '$lib/utils';

export interface CloneInfo {
  org_id: string;
  org_name: string;
  clone_id: string;
}

export interface CloneGroup {
  template_name: string;
  template_id: string | null;
  clones: CloneInfo[];
}

export async function load() {
  if (!store.isLoaded) await store.loadData();
  if (!store.isLoaded) throw redirect(302, '/settings');

  // Get clone detection data
  const orgData: [string, string][] = store.organizations.map(org => [org.id, org.name]);
  const cloneResult = await invoke<{ ok: CloneGroup[] } | InvokeErr>('detect_organization_clones', {
    organizations: orgData,
  });

  let cloneGroups: CloneGroup[] = [];
  if (!isInvokeErr(cloneResult)) {
    cloneGroups = cloneResult.ok;
  }

  // Build a set of clone org IDs for quick lookup
  const cloneOrgIds = new Set<string>();
  for (const group of cloneGroups) {
    for (const clone of group.clones) {
      cloneOrgIds.add(clone.org_id);
    }
  }

  // Get organizations that aren't clones
  const templateOrgs = store.organizations.filter(org => !cloneOrgIds.has(org.id));

  return {
    organizations: store.organizations,
    templateOrgs,
    cloneGroups,
    teams: store.teams,
    owners: store.owners,
  };
}
