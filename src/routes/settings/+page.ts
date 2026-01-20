import { get } from 'svelte/store';
import settings from '$lib/store/settings';

export async function load() {
  await settings.init();
  const data = get(settings);

  return {
    basePath: data.basePath,
    e2ePwd: data.e2ePwd,
    e2eUrl: data.e2eUrl,
    codeEditor: data.codeEditor,
    backofficeApiKey: data.backofficeApiKey,
    theme: data.theme
  };
}
