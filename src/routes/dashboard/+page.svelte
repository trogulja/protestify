<script lang="ts">
  import StatCard from '$lib/ui/StatCard.svelte';

  let { data } = $props();

  let stats = $derived(data.stats);
  let brokenScenarios = $derived(data.brokenScenarios);
  let orgsWithoutTests = $derived(data.orgsWithoutTests);
  let teamBreakdown = $derived(data.teamBreakdown);
  let ownerBreakdown = $derived(data.ownerBreakdown);
</script>

<div class="space-y-8">
  <div>
    <h1 class="text-2xl font-bold mb-6">Dashboard</h1>

    <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4">
      <StatCard
        title="Features"
        value={stats.featureCount}
        variant="info"
      />
      <StatCard
        title="Scenarios"
        value={stats.scenarioCount}
        variant="info"
      />
      <StatCard
        title="Organizations"
        value={stats.organizationCount}
        href="/organizations"
      />
      <StatCard
        title="Teams"
        value={stats.teamCount}
      />
      <StatCard
        title="Owners"
        value={stats.ownerCount}
      />
      <StatCard
        title="Broken Tests"
        value={stats.brokenCount}
        variant={stats.brokenCount > 0 ? 'error' : 'success'}
      />
    </div>
  </div>

  {#if brokenScenarios.length > 0}
    <div>
      <h2 class="text-xl font-semibold mb-4">Broken Tests</h2>
      <div class="card bg-base-100 border shadow-sm">
        <div class="overflow-x-auto">
          <table class="table table-sm">
            <thead>
              <tr>
                <th>Scenario</th>
                <th>Feature</th>
                <th>Organization</th>
                <th>Owner</th>
              </tr>
            </thead>
            <tbody>
              {#each brokenScenarios as scenario}
                <tr class="text-error">
                  <td>
                    <a href="/scenario/{scenario.id}" class="link link-hover">{scenario.name}</a>
                  </td>
                  <td>
                    <a href="/feature/{scenario.featureId}" class="link link-hover">{scenario.featureName}</a>
                  </td>
                  <td>
                    <a href="/organization/{scenario.organizationId}" class="link link-hover">{scenario.organizationName}</a>
                  </td>
                  <td>
                    {#if scenario.organizationOwnerAvatar}
                      <div class="flex items-center gap-2">
                        <div class="avatar">
                          <div class="w-6 rounded">
                            <img src={scenario.organizationOwnerAvatar} alt={scenario.organizationOwner} />
                          </div>
                        </div>
                        <a href="/owner/{scenario.organizationOwner}" class="link link-hover">{scenario.organizationOwner}</a>
                      </div>
                    {:else}
                      <a href="/owner/{scenario.organizationOwner}" class="link link-hover">{scenario.organizationOwner}</a>
                    {/if}
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  {/if}

  <div class="grid md:grid-cols-2 gap-8">
    <div>
      <h2 class="text-xl font-semibold mb-4">By Team</h2>
      <div class="card bg-base-100 border shadow-sm">
        <div class="overflow-x-auto max-h-80">
          <table class="table table-sm">
            <thead class="sticky top-0 bg-base-100">
              <tr>
                <th>Team</th>
                <th class="text-right">Scenarios</th>
                <th class="text-right">Broken</th>
              </tr>
            </thead>
            <tbody>
              {#each teamBreakdown as row}
                <tr>
                  <td>
                    <a href="/team/{row.team}" class="link link-hover">{row.team}</a>
                  </td>
                  <td class="text-right">{row.scenarioCount}</td>
                  <td class="text-right {row.brokenCount > 0 ? 'text-error' : ''}">{row.brokenCount}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>
    </div>

    <div>
      <h2 class="text-xl font-semibold mb-4">By Owner</h2>
      <div class="card bg-base-100 border shadow-sm">
        <div class="overflow-x-auto max-h-80">
          <table class="table table-sm">
            <thead class="sticky top-0 bg-base-100">
              <tr>
                <th>Owner</th>
                <th class="text-right">Scenarios</th>
                <th class="text-right">Broken</th>
              </tr>
            </thead>
            <tbody>
              {#each ownerBreakdown as row}
                <tr>
                  <td>
                    <div class="flex items-center gap-2">
                      {#if row.avatar}
                        <div class="avatar">
                          <div class="w-6 rounded">
                            <img src={row.avatar} alt={row.owner} />
                          </div>
                        </div>
                      {/if}
                      <a href="/owner/{row.owner}" class="link link-hover">{row.owner}</a>
                    </div>
                  </td>
                  <td class="text-right">{row.scenarioCount}</td>
                  <td class="text-right {row.brokenCount > 0 ? 'text-error' : ''}">{row.brokenCount}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>

  {#if orgsWithoutTests.length > 0}
    <div>
      <h2 class="text-xl font-semibold mb-4">Organizations Without Tests</h2>
      <div class="card bg-base-100 border shadow-sm">
        <div class="overflow-x-auto max-h-60">
          <table class="table table-sm">
            <thead class="sticky top-0 bg-base-100">
              <tr>
                <th>ID</th>
                <th>Name</th>
                <th>Team</th>
                <th>Owner</th>
              </tr>
            </thead>
            <tbody>
              {#each orgsWithoutTests as org}
                <tr class="opacity-60">
                  <td>
                    <a href="/organization/{org.id}" class="link link-hover">{org.id}</a>
                  </td>
                  <td>{org.name}</td>
                  <td>
                    <a href="/team/{org.teamName}" class="link link-hover">{org.teamName}</a>
                  </td>
                  <td>
                    <a href="/owner/{org.ownerName}" class="link link-hover">{org.ownerName}</a>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  {/if}
</div>
