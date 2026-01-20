<script lang="ts">
  import type { CloneGroup } from './+page';

  let { data } = $props();

  let searchTerm = $state('');
  let filterTeam = $state('');
  let filterOwner = $state('');
  let showClones = $state(false);

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

  let relevantCloneGroups = $derived.by(() => {
    const filteredNames = new Set(filteredOrgs.map(o => o.name));
    return data.cloneGroups.filter((group: CloneGroup) => filteredNames.has(group.template_name));
  });

  function getScenarioCount(orgName: string): number {
    return data.organizations.filter(o => o.name === orgName).reduce((acc, org) => acc + org.scenarios.length, 0);
  }

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
    <div>
      <h1 class="text-xl font-semibold text-base-content mb-1">Organizations</h1>
      <p class="text-sm text-base-content/60">Manage and explore test organizations</p>
    </div>
    <span class="text-sm text-base-content/50 font-medium">{data.organizations.length} total</span>
  </div>

  <div class="flex flex-wrap items-center gap-3">
    <div class="relative">
      <input
        type="text"
        class="input input-sm w-64 pl-9"
        placeholder="Search organizations..."
        bind:value={searchTerm}
        onkeydown={handleKeydown}
      />
      <svg xmlns="http://www.w3.org/2000/svg" class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-base-content/40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/></svg>
    </div>

    <select class="select select-sm" bind:value={filterTeam}>
      <option value="">All Teams</option>
      {#each data.teams as team}
        <option value={team.name}>{team.name}</option>
      {/each}
    </select>

    <select class="select select-sm" bind:value={filterOwner}>
      <option value="">All Owners</option>
      {#each data.owners as owner}
        <option value={owner.name}>{owner.name}</option>
      {/each}
    </select>

    <label class="flex items-center gap-2 cursor-pointer">
      <input type="checkbox" class="toggle toggle-sm toggle-primary" bind:checked={showClones} />
      <span class="text-sm text-base-content/70">Show clones</span>
    </label>
  </div>

  {#if data.cloneGroups.length > 0}
    <div class="flex items-center gap-2 px-4 py-3 rounded-lg bg-info/10 border border-info/20">
      <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 text-info" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/></svg>
      <span class="text-sm">
        Found <strong>{data.cloneGroups.length}</strong> template organizations with
        <strong>{data.cloneGroups.reduce((acc: number, g: CloneGroup) => acc + g.clones.length, 0)}</strong> test clones
      </span>
    </div>
  {/if}

  <div class="card-clean overflow-hidden">
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
            <tr class={!hasTests(org.name) ? 'text-base-content/50' : ''}>
              <td>
                <a href="/organization/{org.id}" class="font-mono text-xs hover:underline">{org.id}</a>
              </td>
              <td>
                <a href="/organization/{org.id}" class="link-subtle hover:underline">{org.name}</a>
              </td>
              <td>
                <a href="/team/{org.teamName}" class="link-subtle hover:underline">{org.teamName}</a>
              </td>
              <td>
                {#if org.owner?.avatar}
                  <div class="flex items-center gap-2">
                    <img src={org.owner.avatar} alt={org.ownerName} class="w-6 h-6 rounded-full" />
                    <a href="/owner/{org.ownerName}" class="link-subtle hover:underline">{org.ownerName}</a>
                  </div>
                {:else}
                  <a href="/owner/{org.ownerName}" class="link-subtle hover:underline">{org.ownerName}</a>
                {/if}
              </td>
              <td class="text-right">
                {#if scenarioCount > 0}
                  <span class="inline-flex items-center justify-center min-w-[1.5rem] px-1.5 py-0.5 rounded-full text-xs font-medium bg-success/15 text-success">{scenarioCount}</span>
                {:else}
                  <span class="text-base-content/30">0</span>
                {/if}
              </td>
              <td class="text-right">
                {#if cloneGroup}
                  <span class="inline-flex items-center justify-center min-w-[1.5rem] px-1.5 py-0.5 rounded-full text-xs font-medium bg-info/15 text-info">{cloneGroup.clones.length}</span>
                {:else}
                  <span class="text-base-content/30">-</span>
                {/if}
              </td>
            </tr>

            {#if showClones && cloneGroup}
              {#each cloneGroup.clones as clone}
                <tr class="bg-base-200/50">
                  <td class="pl-8">
                    <span class="font-mono text-xs text-base-content/50">{clone.org_id}</span>
                  </td>
                  <td colspan="3" class="text-sm text-base-content/60">
                    <span class="inline-flex items-center px-1.5 py-0.5 rounded text-xs font-medium bg-base-content/10 mr-2">TEMP-{clone.clone_id}</span>
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

  <p class="text-sm text-base-content/50">
    Showing {filteredOrgs.length} of {data.templateOrgs.length} template organizations
  </p>
</div>
