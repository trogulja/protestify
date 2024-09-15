<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { isInvokeErr } from '$lib/utils';
  import { addToast, ToastType } from '$lib/store/toasts';
  import settings from '$lib/store/settings';

  import Icon from '$lib/ui/Icon.svelte';
  import Label from '$lib/ui/form/Label.svelte';

  export let data: {
    basePath: string;
    e2ePwd: string;
    e2eUrl: string;
    codeEditor: string;
  };

  let basePath = data.basePath;
  let e2ePwd = data.e2ePwd;
  let e2eUrl = data.e2eUrl;
  let codeEditor = data.codeEditor;


  async function autoDetect() {
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

  async function saveData() {
    try {
      e2eUrl = e2eUrl ? new URL(e2eUrl).origin : e2eUrl;
    } catch (error) {
      addToast({
        type: ToastType.ERROR,
        message: 'Invalid e2e url!',
      });
    }

    settings.update({basePath, e2ePwd, e2eUrl, codeEditor});

    addToast({
      type: ToastType.SUCCESS,
      message: 'Settings saved!',
    });
  }

  $: isDirty =
    basePath !== $settings.basePath ||
    e2ePwd !== $settings.e2ePwd ||
    e2eUrl !== $settings.e2eUrl ||
    codeEditor !== $settings.codeEditor;
</script>

<div class="max-w-[85rem] px-4 py-10 sm:px-6 lg:px-8 lg:py-14 mx-auto">
  <div class="mx-auto max-w-2xl">
    <div class="text-center">
      <h2 class="text-xl text-gray-800 font-bold sm:text-3xl dark:text-white">
        Settings
      </h2>
    </div>

    <div
      class="mt-5 p-4 relative z-10 bg-white border rounded-xl sm:mt-10 md:p-10 dark:bg-neutral-900 dark:border-neutral-700"
    >
      <form on:submit|preventDefault={saveData}>
        <div class="mb-4 sm:mb-8">
          <Label forId="e2e-repo-location-input">
            E2e repo location
            <div slot="info">
              <h2 class="card-title">Auto-detectable!</h2>
              <p>Full path to the e2e repo location in your file system.</p>
              <p>This app can't work without knowing where to find the feature files.</p>
            </div>
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
            <div slot="info">
              <h2 class="card-title">Auto-detectable!</h2>
              <p>A common password for the e2e users, will be used for opening the target web page.</p>
            </div>
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
            <div slot="info">
              <h2 class="card-title">Auto-detectable!</h2>
              <p>App url (e2e env), will be used for opening the target web page.</p>
            </div>
          </Label>

          <input
            type="text"
            id="e2e-repo-url-input"
            class="input input-bordered w-full"
            bind:value={e2eUrl}
          />
        </div>

        <div class="mb-4 sm:mb-8">
          <Label forId="code-editor-input">
            Code editor
            <div slot="info">
              <h2 class="card-title">Not auto-detectable</h2>
              <p>Code editor used to open the feature file... It should be "code", but you might prefer something else.</p>
            </div>
          </Label>

          <input
            type="text"
            id="code-editor-input"
            class="input input-bordered w-full"
            bind:value={codeEditor}
          />
        </div>

        <div class="mt-6 grid">
          <div class="flex gap-2">
            <button
              type="submit"
              class="btn {isDirty ? 'btn-warning' : 'btn-success'} grow"
              >Save</button
            >
            <button
              class="btn btn-primary btn-square"
              on:click|preventDefault={autoDetect}
            >
              <Icon name="wizard" class="h-6 w-6" />
            </button>
          </div>
        </div>
      </form>
    </div>
    <!-- End Card -->
  </div>
</div>
<!-- End Comment Form -->
