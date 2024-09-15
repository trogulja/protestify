<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  // import SuccessIcon from "./SuccessIcon.svelte";
  // import ErrorIcon from "./ErrorIcon.svelte";
  // import InfoIcon from "./InfoIcon.svelte";
  // import CloseIcon from "./CloseIcon.svelte";

  const dispatch = createEventDispatcher();

  // TODO: add icons for different types
  export let type = 'error';
  export let dismissible = true;
  export let message = 'Something went wrong.';

  $: className = `alert-${type}`;
</script>

<!-- TODO: fix this, tree shaking will not include these classes if they are dynamically constructed -->
<div class="alert-success alert-error alert-info"></div>

<div role="alert" class="alert {className}">
  <!-- {#if type === "success"}
    <SuccessIcon width="1.1em" />
  {:else if type === "error"}
    <ErrorIcon width="1.1em" />
  {:else}
    <InfoIcon width="1.1em" />
  {/if} -->
  <span>{message}</span>
  {#if dismissible}
    <button class="close" on:click={() => dispatch("dismiss")}>
      X
      <!-- <CloseIcon width="0.8em" /> -->
    </button>
  {/if}
</div>
