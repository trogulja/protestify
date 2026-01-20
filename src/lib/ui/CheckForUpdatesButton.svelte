<script lang="ts">
  import type { Update } from '@tauri-apps/plugin-updater';
  import { message } from '@tauri-apps/plugin-dialog';
  import { relaunch } from '@tauri-apps/plugin-process';

  let { update }: { update: Update } = $props();

  let isInstalling = $state(false);

  async function installUpdate() {
    isInstalling = true;

    try {
      await update.downloadAndInstall();
      await message('Update installed successfully. Relaunching the app...', {
        title: 'Success',
        kind: 'info',
        okLabel: 'OK',
      });
      await relaunch();
    } catch (error) {
      isInstalling = false;
      const errorMessage = error instanceof Error ? error.message : String(error);
      await message(`Failed to install update:\n\n${errorMessage}`, {
        title: 'Error',
        kind: 'error',
        okLabel: 'OK',
      });
    }
  }
</script>

<button
  onclick={installUpdate}
  class="btn btn-primary btn-sm"
  disabled={isInstalling}
>
  {#if isInstalling}
    <span class="loading loading-spinner loading-xs"></span>
    installing...
  {:else}
    update to {update.version}
  {/if}
</button>
