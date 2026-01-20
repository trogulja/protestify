<script lang="ts">
  import { invalidateAll } from '$app/navigation';
  import store from '$lib/store';
  import searchTerm from '$lib/store/search-term';
  import Icon from '$lib/ui/Icon.svelte';

  let { data } = $props();
  let tableData = $derived(data.tableData);

  let filteredTableData = $derived(tableData.filter((row) => {
    return (
      row.scenario.toLowerCase().includes($searchTerm.toLowerCase()) ||
      row.feature.toLowerCase().includes($searchTerm.toLowerCase()) ||
      row.organization.toLowerCase().includes($searchTerm.toLowerCase()) ||
      row.organizationId.toLowerCase().includes($searchTerm.toLowerCase()) ||
      row.tags.some((tag) => tag.toLowerCase().includes($searchTerm.toLowerCase())) ||
      row.owner.toLowerCase().includes($searchTerm.toLowerCase()) ||
      row.team.toLowerCase().includes($searchTerm.toLowerCase())
    );
  }));

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Escape") {
      $searchTerm = "";
    }
  }

  async function handleReloadData() {
    await store.reloadData();
    await invalidateAll();
  }

  // @ts-expect-error - for debugging
  window.p = store;
</script>

<div class="space-y-6">
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-xl font-semibold text-base-content mb-1">Tests</h1>
      <p class="text-sm text-base-content/60">Browse scenarios and features</p>
    </div>
    <span class="text-sm text-base-content/50 font-medium">{tableData.length} scenarios</span>
  </div>

  <div class="flex items-center gap-3">
    <div class="relative flex-1 max-w-md">
      <input
        type="text"
        class="input input-sm w-full pl-9"
        placeholder="Search scenarios, features, tags..."
        bind:value={$searchTerm}
        onkeydown={handleKeydown}
      />
      <svg xmlns="http://www.w3.org/2000/svg" class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-base-content/40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/></svg>
    </div>

    <button
      class="btn btn-sm btn-ghost"
      onclick={handleReloadData}
      title="Reload data"
    >
      <Icon name="reload" class="h-4 w-4" />
    </button>
  </div>

  <div class="card-clean overflow-hidden">
    <div class="overflow-x-auto">
      <table class="table table-sm">
        <thead>
          <tr>
            <th class="w-12">Owner</th>
            <th>Scenario</th>
            <th>Feature</th>
            <th>Organization</th>
            <th>Team</th>
            <th class="text-right">Tags</th>
          </tr>
        </thead>
        <tbody>
          {#each filteredTableData as td}
            <tr class={td.isBroken ? 'text-error' : ''}>
              <td class="text-center">
                {#if td.ownerAvatar}
                  <a href="/owner/{td.owner}">
                    <img src={td.ownerAvatar} alt={td.owner} class="w-7 h-7 rounded-full mx-auto" />
                  </a>
                {:else}
                  <a href="/owner/{td.owner}" class="text-xs link-subtle hover:underline">{td.owner.slice(0, 2)}</a>
                {/if}
              </td>
              <td>
                <a href="/scenario/{td.scenarioId}" class="link-subtle hover:underline {td.isBroken ? 'text-error' : ''}">{td.scenario}</a>
              </td>
              <td>
                <a href="/feature/{td.featureId}" class="link-subtle hover:underline">{td.feature}</a>
              </td>
              <td>
                <a href="/organization/{td.organizationId}" class="link-subtle hover:underline font-mono text-xs" title={td.organization}>{td.organizationId}</a>
              </td>
              <td>
                <a href="/team/{td.team}" class="link-subtle hover:underline">{td.team}</a>
              </td>
              <td class="text-right">
                <div class="flex flex-wrap justify-end gap-1">
                  {#each td.tags as tag}
                    <span class="inline-flex items-center px-1.5 py-0.5 rounded text-xs font-medium {tag.includes('broken') ? 'bg-error/15 text-error' : 'bg-base-content/10 text-base-content/70'}">{tag}</span>
                  {/each}
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>

  {#if $searchTerm}
    <p class="text-sm text-base-content/50">
      Showing {filteredTableData.length} of {tableData.length} scenarios
    </p>
  {/if}
</div>
