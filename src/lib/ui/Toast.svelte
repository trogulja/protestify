<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  // import SuccessIcon from "./SuccessIcon.svelte";
  // import ErrorIcon from "./ErrorIcon.svelte";
  // import InfoIcon from "./InfoIcon.svelte";
  // import CloseIcon from "./CloseIcon.svelte";

  const dispatch = createEventDispatcher();

  
  interface Props {
    // TODO: add icons for different types
    type?: string;
    dismissible?: boolean;
    message?: string;
  }

  let { type = 'error', dismissible = true, message = 'Something went wrong.' }: Props = $props();

  let className = $derived(`alert-${type}`);
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
    <button class="close" onclick={() => dispatch("dismiss")}>
      X
      <!-- <CloseIcon width="0.8em" /> -->
    </button>
  {/if}
</div>
