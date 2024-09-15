<script lang="ts">
  import Toast from "$lib/ui/Toast.svelte";

  import { dismissToast, toasts } from "$lib/store/toasts";
</script>

{#if $toasts}
  <div class="toast toast-center z-50">
    {#each $toasts as toast (toast.id)}
      <Toast
        type={toast.type}
        dismissible={toast.dismissible}
        message={toast.message}
        on:dismiss={() => dismissToast(toast.id)}
      />
    {/each}
  </div>
{/if}
