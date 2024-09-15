<script lang="ts">
  import Icon from '$lib/ui/Icon.svelte';
  import type { BCType } from '$lib/ui/Breadcrumbs.svelte';

  export let link: string | null = null;
  export let name: string;
  export let type: BCType;
</script>

{#if link}
  <li>
    <a href={link} class="inline-flex items-center gap-2">
      <Icon
        name={type}
        class="h-4 w-4"
      />
      {name}
    </a>
  </li>
{:else}
  <li>
    <span class="inline-flex items-center gap-2">
      <Icon
        name={type}
        class="h-4 w-4"
      />
      {name}
    </span>
  </li>
{/if}
