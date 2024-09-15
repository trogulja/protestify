<script lang="ts">
  import Icon from '$lib/ui/Icon.svelte';

  export let forId = "";
</script>

<label for={forId} class="block mb-2 text-sm font-medium dark:text-white">
  <slot></slot>
  {#if $$slots.info}
    <div class="dropdown dropdown-hover">
      <div tabindex="-1" role="button" class="btn btn-circle btn-ghost btn-xs text-info">
        <Icon name="question" class="h-4 w-4 stroke-current" tabindex="-1" />
      </div>
      <div tabindex="-1" class="card compact dropdown-content bg-base-100 rounded-box z-[1] w-64 shadow">
        <div tabindex="-1" class="card-body">
          <slot name="info"></slot>
        </div>
      </div>
    </div>
  {/if}
</label>
