<script lang="ts">
  import type { CloneGroup } from './+page';

  let { data } = $props();

  let searchTerm = $state('');
  let filterTeam = $state('');
  let filterOwner = $state('');
  let showClones = $state(false);

  // Filter template organizations
  let filteredOrgs = $derived(
    data.templateOrgs.filter((org) => {
      const matchesSearch =
        org.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
        org.id.toLowerCase().includes(searchTerm.toLowerCase());
      const matchesTeam = !filterTeam || org.teamName === filterTeam;
      const matchesOwner = !filterOwner || org.ownerName === filterOwner;
      return matchesSearch && matchesTeam && matchesOwner;
    })
  );

  // Get clone groups that match filtered orgs
  let relevantCloneGroups = $derived.by(() => {
    const filteredNames = new Set(filteredOrgs.map(o => o.name));
    return data.cloneGroups.filter((group: CloneGroup) => filteredNames.has(group.template_name));
  });

  // Count scenarios per org
  function getScenarioCount(orgName: string): number {
    return data.organizations.filter(o => o.name === orgName).reduce((acc, org) => acc + org.scenarios.length, 0);
  }

  // Check if org has any scenarios
  function hasTests(orgName: string): boolean {
    return getScenarioCount(orgName) > 0;
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      searchTerm = '';
    }
  }
</script>

<div class="space-y-6">
  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-bold">Organizations</h1>
    <div class="badge badge-neutral">{data.organizations.length} total</div>
  </div>

  <div class="flex flex-wrap gap-4">
    <label class="input input-bordered input-sm flex items-center gap-2 w-64">
      <input
        type="text"
        class="grow"
        placeholder="Search organizations..."
        bind:value={searchTerm}
        onkeydown={handleKeydown}
      />
      <svg xmlns="http://www.w3.org/2000/svg" width="1.2em" height="1.2em" viewBox="0 0 24 24"><path fill="currentColor" d="m19.485 20.154l-6.262-6.262q-.75.639-1.725.989t-1.96.35q-2.402 0-4.066-1.663T3.808 9.503T5.47 5.436t4.064-1.667t4.068 1.664T15.268 9.5q0 1.042-.369 2.017t-.97 1.668l6.262 6.261zM9.539 14.23q1.99 0 3.36-1.37t1.37-3.361t-1.37-3.36t-3.36-1.37t-3.361 1.37t-1.37 3.36t1.37 3.36t3.36 1.37"/></svg>
    </label>

    <select class="select select-bordered select-sm" bind:value={filterTeam}>
      <option value="">All Teams</option>
      {#each data.teams as team}
        <option value={team.name}>{team.name}</option>
      {/each}
    </select>

    <select class="select select-bordered select-sm" bind:value={filterOwner}>
      <option value="">All Owners</option>
      {#each data.owners as owner}
        <option value={owner.name}>{owner.name}</option>
      {/each}
    </select>

    <label class="label cursor-pointer gap-2">
      <input type="checkbox" class="toggle toggle-sm" bind:checked={showClones} />
      <span class="label-text">Show clones</span>
    </label>
  </div>

  {#if data.cloneGroups.length > 0}
    <div class="alert alert-info">
      <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 24 24"><path fill="currentColor" d="M11 17h2v-6h-2zm1-8q.425 0 .713-.288T13 8t-.288-.712T12 7t-.712.288T11 8t.288.713T12 9m0 13q-2.075 0-3.9-.788t-3.175-2.137T2.788 15.9T2 12t.788-3.9t2.137-3.175T8.1 2.788T12 2t3.9.788t3.175 2.137T21.213 8.1T22 12t-.788 3.9t-2.137 3.175t-3.175 2.138T12 22"/></svg>
      <span>
        Found <strong>{data.cloneGroups.length}</strong> template organizations with
        <strong>{data.cloneGroups.reduce((acc: number, g: CloneGroup) => acc + g.clones.length, 0)}</strong> test clones
      </span>
    </div>
  {/if}

  <div class="card bg-base-100 border shadow-sm">
    <div class="overflow-x-auto">
      <table class="table table-sm">
        <thead>
          <tr>
            <th>ID</th>
            <th>Name</th>
            <th>Team</th>
            <th>Owner</th>
            <th class="text-right">Tests</th>
            <th class="text-right">Clones</th>
          </tr>
        </thead>
        <tbody>
          {#each filteredOrgs as org}
            {@const cloneGroup = relevantCloneGroups.find((g: CloneGroup) => g.template_name === org.name)}
            {@const scenarioCount = getScenarioCount(org.name)}
            <tr class:opacity-50={!hasTests(org.name)}>
              <td>
                <a href="/organization/{org.id}" class="link link-hover font-mono">{org.id}</a>
              </td>
              <td>
                <a href="/organization/{org.id}" class="link link-hover">{org.name}</a>
              </td>
              <td>
                <a href="/team/{org.teamName}" class="link link-hover">{org.teamName}</a>
              </td>
              <td>
                {#if org.owner?.avatar}
                  <div class="flex items-center gap-2">
                    <div class="avatar">
                      <div class="w-6 rounded">
                        <img src={org.owner.avatar} alt={org.ownerName} />
                      </div>
                    </div>
                    <a href="/owner/{org.ownerName}" class="link link-hover">{org.ownerName}</a>
                  </div>
                {:else}
                  <a href="/owner/{org.ownerName}" class="link link-hover">{org.ownerName}</a>
                {/if}
              </td>
              <td class="text-right">
                {#if scenarioCount > 0}
                  <span class="badge badge-success badge-sm">{scenarioCount}</span>
                {:else}
                  <span class="badge badge-ghost badge-sm">0</span>
                {/if}
              </td>
              <td class="text-right">
                {#if cloneGroup}
                  <span class="badge badge-info badge-sm">{cloneGroup.clones.length}</span>
                {:else}
                  -
                {/if}
              </td>
            </tr>

            {#if showClones && cloneGroup}
              {#each cloneGroup.clones as clone}
                <tr class="bg-base-200/50">
                  <td class="pl-8">
                    <span class="font-mono text-xs opacity-70">{clone.org_id}</span>
                  </td>
                  <td colspan="3" class="text-sm opacity-70">
                    <span class="badge badge-outline badge-xs mr-2">TEMP-{clone.clone_id}</span>
                    {clone.org_name}
                  </td>
                  <td></td>
                  <td></td>
                </tr>
              {/each}
            {/if}
          {/each}
        </tbody>
      </table>
    </div>
  </div>

  <div class="text-sm opacity-60">
    Showing {filteredOrgs.length} of {data.templateOrgs.length} template organizations
  </div>
</div>
