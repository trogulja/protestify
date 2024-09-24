<script lang="ts">
  import { check } from '@tauri-apps/plugin-updater';
  import { ask, message } from '@tauri-apps/plugin-dialog';
  import { relaunch } from '@tauri-apps/plugin-process';

  async function checkForAppUpdates() {
    let update;
    let errorMessage;

    try {
      update = await check();
    } catch (error) {
      if (error instanceof Error) {
        errorMessage = error.message;
      } else if (typeof error === 'string') {
        errorMessage = error;
      } else {
        errorMessage = JSON.stringify(error);
      }
    }

    if (errorMessage) {
      await message(`Failed to check for updates:\n\n${errorMessage}`, {
        title: 'Error',
        kind: 'error',
        okLabel: 'OK',
      });
      return;
    } else if (update?.available) {
      const yes = await ask(
        `Update to ${update.version} is available. Do you want to update now?\n\nRelease notes: ${update.body}`,
        {
          title: 'Update available',
          kind: 'info',
          okLabel: 'Yes',
          cancelLabel: 'No',
        }
      );

      if (yes) {
        await update.downloadAndInstall();
        await message('Update installed successfully. Relaunching the app...', {
          title: 'Success',
          kind: 'info',
          okLabel: 'OK',
        });
        await relaunch();
      }
    } else {
      await message('No updates available.', {
        title: 'Info',
        kind: 'info',
        okLabel: 'OK',
      });
    }
  }
</script>

<button on:click={checkForAppUpdates} class="btn btn-primary btn-sm">
  update
</button>
