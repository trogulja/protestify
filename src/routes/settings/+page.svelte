<script lang="ts">
  import { untrack } from 'svelte';
  import { invoke } from "@tauri-apps/api/core";
  import { isInvokeErr } from '$lib/utils';
  import { addToast, ToastType } from '$lib/store/toasts';
  import settings, { type Theme } from '$lib/store/settings';

  import Icon from '$lib/ui/Icon.svelte';
  import Label from '$lib/ui/form/Label.svelte';
  import SettingsTabs from '$lib/ui/SettingsTabs.svelte';

  let { data } = $props();

  const tabs = [
    { id: 'e2e', label: 'E2E Repository', icon: 'folder' },
    { id: 'api', label: 'API', icon: 'key' },
    { id: 'preferences', label: 'Preferences', icon: 'settings' },
  ];

  let activeTab = $state('e2e');

  // Form fields intentionally capture initial values (user edits them locally)
  let basePath = $state(untrack(() => data.basePath));
  let e2ePwd = $state(untrack(() => data.e2ePwd));
  let e2eUrl = $state(untrack(() => data.e2eUrl));
  let codeEditor = $state(untrack(() => data.codeEditor));
  let backofficeApiKey = $state(untrack(() => data.backofficeApiKey));
  let theme = $state<Theme>(untrack(() => data.theme));

  async function autoDetect(event: Event) {
    event.preventDefault();

    const autodetect = await invoke<InvokeFindE2eRepo | InvokeErr>('find_e2e_repo');
    if (isInvokeErr(autodetect)) {
      addToast({
        type: ToastType.ERROR,
        message: `Auto-detect failed: ${autodetect.err}`,
      });
      return;
    }

    addToast({
      type: ToastType.SUCCESS,
      message: 'Auto-detect successful!',
    });

    basePath = autodetect.ok.location;
    e2ePwd = autodetect.ok.password;
    e2eUrl = autodetect.ok.app_url;
  }

  async function saveData(event: Event) {
    event.preventDefault();

    let normalizedE2eUrl = e2eUrl;
    try {
      normalizedE2eUrl = e2eUrl ? new URL(e2eUrl).origin : e2eUrl;
    } catch (error) {
      addToast({
        type: ToastType.ERROR,
        message: 'Invalid e2e url!',
      });
      return;
    }

    settings.update({
      basePath,
      e2ePwd,
      e2eUrl: normalizedE2eUrl,
      codeEditor,
      backofficeApiKey,
      theme
    });

    addToast({
      type: ToastType.SUCCESS,
      message: 'Settings saved!',
    });
  }

  let isDirty = $derived(
    basePath !== $settings.basePath ||
    e2ePwd !== $settings.e2ePwd ||
    e2eUrl !== $settings.e2eUrl ||
    codeEditor !== $settings.codeEditor ||
    backofficeApiKey !== $settings.backofficeApiKey ||
    theme !== $settings.theme
  );
</script>

<div class="max-w-[85rem] px-4 py-10 sm:px-6 lg:px-8 lg:py-14 mx-auto">
  <div class="mx-auto max-w-2xl">
    <div class="text-center mb-8">
      <h2 class="text-xl text-gray-800 font-bold sm:text-3xl dark:text-white">
        Settings
      </h2>
    </div>

    <SettingsTabs {tabs} bind:activeTab />

    <div
      class="p-4 relative z-10 bg-white border rounded-xl md:p-10 dark:bg-neutral-900 dark:border-neutral-700"
    >
      <form onsubmit={saveData}>
        {#if activeTab === 'e2e'}
          <div class="mb-4 sm:mb-8">
            <Label forId="e2e-repo-location-input">
              E2e repo location
              {#snippet info()}
                <div>
                  <h2 class="card-title">Auto-detectable!</h2>
                  <p>Full path to the e2e repo location in your file system.</p>
                  <p>This app can't work without knowing where to find the feature files.</p>
                </div>
              {/snippet}
            </Label>

            <input
              type="text"
              id="e2e-repo-location-input"
              class="input input-bordered w-full"
              bind:value={basePath}
            />
          </div>

          <div class="mb-4 sm:mb-8">
            <Label forId="e2e-repo-password-input">
              E2e password
              {#snippet info()}
                <div>
                  <h2 class="card-title">Auto-detectable!</h2>
                  <p>A common password for the e2e users, will be used for opening the target web page.</p>
                </div>
              {/snippet}
            </Label>

            <input
              type="text"
              id="e2e-repo-password-input"
              class="input input-bordered w-full"
              bind:value={e2ePwd}
            />
          </div>

          <div class="mb-4 sm:mb-8">
            <Label forId="e2e-repo-url-input">
              E2e url
              {#snippet info()}
                <div>
                  <h2 class="card-title">Auto-detectable!</h2>
                  <p>App url (e2e env), will be used for opening the target web page.</p>
                </div>
              {/snippet}
            </Label>

            <input
              type="text"
              id="e2e-repo-url-input"
              class="input input-bordered w-full"
              bind:value={e2eUrl}
            />
          </div>

          <div class="mt-6 grid">
            <div class="flex gap-2">
              <button
                type="submit"
                class="btn {isDirty ? 'btn-warning' : 'btn-success'} grow"
              >Save</button>
              <button
                type="button"
                class="btn btn-primary btn-square"
                onclick={autoDetect}
              >
                <Icon name="wizard" class="h-6 w-6" />
              </button>
            </div>
          </div>
        {:else if activeTab === 'api'}
          <div class="mb-4 sm:mb-8">
            <Label forId="backoffice-api-key-input">
              Backoffice API Key
              {#snippet info()}
                <div>
                  <h2 class="card-title">Optional</h2>
                  <p>API key for connecting to the e2e backoffice service.</p>
                  <p>This enables fetching organization data and test run history.</p>
                </div>
              {/snippet}
            </Label>

            <input
              type="password"
              id="backoffice-api-key-input"
              class="input input-bordered w-full"
              placeholder="Enter your API key"
              bind:value={backofficeApiKey}
            />
          </div>

          <div class="mt-6 grid">
            <button
              type="submit"
              class="btn {isDirty ? 'btn-warning' : 'btn-success'}"
            >Save</button>
          </div>
        {:else if activeTab === 'preferences'}
          <div class="mb-4 sm:mb-8">
            <Label forId="code-editor-input">
              Code editor
              {#snippet info()}
                <div>
                  <h2 class="card-title">Not auto-detectable</h2>
                  <p>Code editor used to open the feature file... It should be "code", but you might prefer something else.</p>
                </div>
              {/snippet}
            </Label>

            <input
              type="text"
              id="code-editor-input"
              class="input input-bordered w-full"
              bind:value={codeEditor}
            />
          </div>

          <div class="mb-4 sm:mb-8">
            <Label forId="theme-select">
              Theme
              {#snippet info()}
                <div>
                  <h2 class="card-title">Appearance</h2>
                  <p>Choose how the app should look. System will follow your OS preference.</p>
                </div>
              {/snippet}
            </Label>

            <select
              id="theme-select"
              class="select select-bordered w-full"
              bind:value={theme}
            >
              <option value="system">System</option>
              <option value="light">Light</option>
              <option value="dark">Dark</option>
            </select>
          </div>

          <div class="mt-6 grid">
            <button
              type="submit"
              class="btn {isDirty ? 'btn-warning' : 'btn-success'}"
            >Save</button>
          </div>
        {/if}
      </form>
    </div>
  </div>
</div>
