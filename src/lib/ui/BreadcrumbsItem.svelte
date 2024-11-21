<script lang="ts">
  import Icon from '$lib/ui/Icon.svelte';
  import type { BCType } from '$lib/ui/Breadcrumbs.svelte';

  interface Props {
    link?: string | null;
    name: string;
    type: BCType;
  }

  let { link = null, name, type }: Props = $props();
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
