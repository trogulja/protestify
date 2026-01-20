<script lang="ts">
  import StatCard from '$lib/ui/StatCard.svelte';

  let { data } = $props();

  let stats = $derived(data.stats);
  let brokenScenarios = $derived(data.brokenScenarios);
  let orgsWithoutTests = $derived(data.orgsWithoutTests);
  let teamBreakdown = $derived(data.teamBreakdown);
  let ownerBreakdown = $derived(data.ownerBreakdown);
</script>

<div class="space-y-6">
  <div>
    <h1 class="text-xl font-semibold text-base-content mb-1">Dashboard</h1>
    <p class="text-sm text-base-content/60">Overview of your e2e test suite</p>
  </div>

  <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-3 gap-4">
    <StatCard
      title="Features"
      value={stats.featureCount}
      subtitle="{stats.scenarioCount} scenarios"
      href="/"
    />
    <StatCard
      title="Organizations"
      value={stats.organizationCount}
      subtitle="{stats.teamCount} teams, {stats.ownerCount} owners"
      href="/organizations"
    />
    <StatCard
      title="Issues"
      value={stats.brokenCount}
      variant={stats.brokenCount > 0 ? 'error' : 'success'}
    />
  </div>

  {#if brokenScenarios.length > 0}
    <div>
      <h2 class="section-title">Broken Tests</h2>
      <div class="card-clean overflow-hidden">
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
                <tr>
                  <td>
                    <a href="/scenario/{scenario.id}" class="text-error hover:underline">{scenario.name}</a>
                  </td>
                  <td>
                    <a href="/feature/{scenario.featureId}" class="link-subtle hover:underline">{scenario.featureName}</a>
                  </td>
                  <td>
                    <a href="/organization/{scenario.organizationId}" class="link-subtle hover:underline">{scenario.organizationName}</a>
                  </td>
                  <td>
                    {#if scenario.organizationOwnerAvatar}
                      <div class="flex items-center gap-2">
                        <img src={scenario.organizationOwnerAvatar} alt={scenario.organizationOwner} class="w-6 h-6 rounded-full" />
                        <a href="/owner/{scenario.organizationOwner}" class="link-subtle hover:underline">{scenario.organizationOwner}</a>
                      </div>
                    {:else}
                      <a href="/owner/{scenario.organizationOwner}" class="link-subtle hover:underline">{scenario.organizationOwner}</a>
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

  <div class="grid md:grid-cols-2 gap-6">
    <div class="flex flex-col">
      <h2 class="section-title">By Team</h2>
      <div class="card-clean overflow-hidden flex-1">
        <div class="overflow-x-auto max-h-80">
          <table class="table table-sm">
            <thead class="sticky top-0">
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
                    <a href="/team/{row.team}" class="link-subtle hover:underline">{row.team}</a>
                  </td>
                  <td class="text-right font-medium">{row.scenarioCount}</td>
                  <td class="text-right {row.brokenCount > 0 ? 'text-error font-medium' : 'text-base-content/40'}">{row.brokenCount}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>
    </div>

    <div class="flex flex-col">
      <h2 class="section-title">By Owner</h2>
      <div class="card-clean overflow-hidden flex-1">
        <div class="overflow-x-auto max-h-80">
          <table class="table table-sm">
            <thead class="sticky top-0">
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
                        <img src={row.avatar} alt={row.owner} class="w-6 h-6 rounded-full" />
                      {/if}
                      <a href="/owner/{row.owner}" class="link-subtle hover:underline">{row.owner}</a>
                    </div>
                  </td>
                  <td class="text-right font-medium">{row.scenarioCount}</td>
                  <td class="text-right {row.brokenCount > 0 ? 'text-error font-medium' : 'text-base-content/40'}">{row.brokenCount}</td>
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
      <h2 class="section-title">Organizations Without Tests</h2>
      <div class="card-clean overflow-hidden">
        <div class="overflow-x-auto max-h-60">
          <table class="table table-sm">
            <thead class="sticky top-0">
              <tr>
                <th>ID</th>
                <th>Name</th>
                <th>Team</th>
                <th>Owner</th>
              </tr>
            </thead>
            <tbody>
              {#each orgsWithoutTests as org}
                <tr class="text-base-content/50">
                  <td>
                    <a href="/organization/{org.id}" class="hover:underline font-mono text-xs">{org.id}</a>
                  </td>
                  <td>{org.name}</td>
                  <td>
                    <a href="/team/{org.teamName}" class="hover:underline">{org.teamName}</a>
                  </td>
                  <td>
                    <a href="/owner/{org.ownerName}" class="hover:underline">{org.ownerName}</a>
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
