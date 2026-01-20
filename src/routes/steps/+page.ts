import { redirect } from '@sveltejs/kit';
import { get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import settings from '$lib/store/settings';
import { isInvokeErr } from '$lib/utils';
import type { StepDefinition } from '$lib/model/step';

export async function load() {
  await settings.init();
  const { basePath } = get(settings);

  if (!basePath) {
    throw redirect(302, '/settings');
  }

  const result = await invoke<{ ok: StepDefinition[] } | InvokeErr>('get_steps', {
    basePath: `${basePath}/features`,
  });

  if (isInvokeErr(result)) {
    return {
      steps: [],
      error: result.err,
    };
  }

  return {
    steps: result.ok,
    error: null,
  };
}
