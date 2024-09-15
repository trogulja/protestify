<script lang="ts">
  import store from '$lib/store';
  import searchTerm from '$lib/store/search-term';
  import Icon from '$lib/ui/Icon.svelte';

  export let data: { tableData: typeof store['tableData']['scenarios'] };
  let tableData = data.tableData;

  $: filteredTableData = tableData.filter((data) => {
    return (
      data.scenario.toLowerCase().includes($searchTerm.toLowerCase()) ||
      data.feature.toLowerCase().includes($searchTerm.toLowerCase()) ||
      data.organization.toLowerCase().includes($searchTerm.toLowerCase()) ||
      data.organizationId.toLowerCase().includes($searchTerm.toLowerCase()) ||
      data.tags.some((tag) => tag.toLowerCase().includes($searchTerm.toLowerCase())) ||
      data.owner.toLowerCase().includes($searchTerm.toLowerCase()) ||
      data.team.toLowerCase().includes($searchTerm.toLowerCase())
    );
  });

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Escape") {
      $searchTerm = "";
    }
  }

  async function handleReloadData() {
    await store.reloadData();
    tableData = store.tableData.scenarios;
  }

  // @ts-expect-error - for debugging
  window.p = store;
</script>

<div class="container mb-4 flex gap-4">
  <label class="input input-bordered input-sm flex items-center gap-2">
    <input
      type="text"
      class="grow"
      placeholder="Search"
      bind:value={$searchTerm}
      on:keydown={handleKeydown}
    />
    <svg xmlns="http://www.w3.org/2000/svg" width="1.3em" height="1.3em" viewBox="0 0 24 24"><path fill="currentColor" d="m19.485 20.154l-6.262-6.262q-.75.639-1.725.989t-1.96.35q-2.402 0-4.066-1.663T3.808 9.503T5.47 5.436t4.064-1.667t4.068 1.664T15.268 9.5q0 1.042-.369 2.017t-.97 1.668l6.262 6.261zM9.539 14.23q1.99 0 3.36-1.37t1.37-3.361t-1.37-3.36t-3.36-1.37t-3.361 1.37t-1.37 3.36t1.37 3.36t3.36 1.37"/></svg>
  </label>

  <div class="tooltip" data-tip="Reload data">
    <button
      class="btn btn-sm btn-neutral btn-square"
      on:click={handleReloadData}
    >
      <Icon
        name="reload"
        class="h-4 w-4"
      />
    </button>
  </div>
</div>

<div>
  <table class="table table-xs">
    <thead>
      <tr>
        <th>owner</th>
        <th>scenario</th>
        <th>feature</th>
        <th>organization</th>
        <th>team</th>
        <th>tags</th>
      </tr>
    </thead>
    <tbody>
      {#each filteredTableData as td}
        <tr class:text-error={td.isBroken}>
          <td class="text-center">
            {#if td.ownerAvatar}
              <div class="avatar">
                <div class="w-8 rounded">
                  <a href="/owner/{td.owner}"><img src="{td.ownerAvatar}" alt="{td.owner}" /></a>
                </div>
              </div>
            {:else}
              <a href="/owner/{td.owner}" class="link link-hover">{td.owner}</a>
            {/if}
          </td>
          <td>
            <a href="/scenario/{td.scenarioId}" class="link link-hover">{td.scenario}</a>
          </td>
          <td>
            <a href="/feature/{td.featureId}" class="link link-hover">{td.feature}</a>
          </td>
          <td class="text-center">
            <div class="tooltip tooltip-top" data-tip={td.organization}>
              <a href="/organization/{td.organizationId}" class="link link-hover">{td.organizationId}</a>
            </div>
          </td>
          <td>
            <a href="/team/{td.team}" class="link link-hover">{td.team}</a>
          </td>
          <td class="text-right">
            {#each td.tags as tag}
              <span class="badge {tag.includes('broken') ? 'badge-error' : 'badge-neutral'} badge-sm">{tag}</span>
            {/each}
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>
