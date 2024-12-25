<script lang="ts">
  import type {CommandCollection} from '$lib/ui/CommandPreview.svelte';

  import Breadcrumbs from '$lib/ui/Breadcrumbs.svelte';
  import Icon from '$lib/ui/Icon.svelte';
  import CommandPreview from '$lib/ui/CommandPreview.svelte';
  import FileContents from '$lib/ui/FileContents.svelte';
  import OrganizationTargets from '$lib/ui/OrganizationTargets.svelte';
  import TestRunner from '$lib/ui/TestRunner.svelte';

  let { data } = $props();

  const scenario = data.scenario;
  const filePath = data.filePath;
  const scenarioName = data.scenarioName;

  // TODO: handle this via some class and state
  let targetCmd: CommandCollection = $state({isEmptyCommand: true});
  let testRunnerCmd: CommandCollection = $state({isEmptyCommand: true});
  let previewCmd: CommandCollection = $state({isEmptyCommand: true});

  const setPreviewCmd = (cmd: CommandCollection) => {
    previewCmd = cmd;
  }

  $effect(() => {
    setPreviewCmd(targetCmd);
  })

  $effect(() => {
    setPreviewCmd(testRunnerCmd);
  })

  // @ts-expect-error - for debugging
  window.k = scenario;
</script>

<Breadcrumbs
  name={scenario.name}
  type="scenario"
  parent={scenario.featureName}
  parentUrl="/feature/{scenario.featureId}"
/>

<div class="flex gap-6 mb-6">
  <div class="flex-1 card bg-neutral w-96 shadow-xl">
    <div class="card-body">
      <OrganizationTargets
        bind:command={targetCmd}
        targets={scenario.targets}
        users={scenario.organizationUsers}
        slug={scenario.organizationSlug}
      />

      <div class="divider"></div>

      <TestRunner
        bind:command={testRunnerCmd}
        scenarioName={scenarioName}
        featureFile={filePath}
      />

      <div class="divider"></div>

      <CommandPreview
        command={previewCmd}
        maxLineLength={scenario.commandMaxLen}
      />
    </div>

  </div>

  <div class="flex-1 card bg-neutral w-96 shadow-xl">
    <div class="card-body">
      <h2 class="card-title text-white mb-6">{scenario.name}</h2>

      <div>
        <Icon
          name="feature"
          class="h-5 w-5 mr-2 inline-block align-text-bottom"
        />
        <a href="/feature/{scenario.featureId}" class="link link-hover link-primary">
          {scenario.featureName}
        </a>
      </div>
      <div>
        <Icon
          name="organization"
          class="h-5 w-5 mr-2 inline-block align-text-bottom"
        />
        <a href="/organization/{scenario.organizationId}" class="link link-hover link-primary">
          {scenario.organizationName}
        </a>
      </div>
      <div>
        <Icon
          name="person"
          class="h-5 w-5 mr-2 inline-block align-text-bottom"
        />
        <a href="/owner/{scenario.organizationOwner}" class="link link-hover link-primary">
          {scenario.organizationOwner}
        </a>
      </div>
      <div>
        <Icon
          name="team"
          class="h-5 w-5 mr-2 inline-block align-text-bottom"
        />
        <a href="/team/{scenario.organizationTeam}" class="link link-hover link-primary">
          {scenario.organizationTeam}
        </a>
      </div>

      <div class="divider"></div>

      <div>
        <Icon
          name="target"
          class="h-5 w-5 mr-2 inline-block align-text-bottom"
        />
        {scenario.targets.length} target screens
      </div>
      <div>
        <Icon
          name="team"
          class="h-5 w-5 mr-2 inline-block align-text-bottom"
        />
        {scenario.organizationUsersCount} defined users
      </div>
      <div>
        <Icon
          name="flag"
          class="h-5 w-5 mr-2 inline-block align-text-bottom"
        />
        {scenario.flags.length} flags
      </div>
      <div>
        <Icon
          name="date"
          class="h-5 w-5 mr-2 inline-block align-text-bottom"
        />
        {scenario.mockDates.length} mock date
      </div>
      <div>
        <Icon
          name="tag"
          class="h-5 w-5 mr-2 inline-block align-text-bottom"
        />
        {scenario.allTags.length} tags {scenario.allTags.join(', ')}
      </div>
    </div>

    {#if scenario.isBroken}
      <div class="card-actions justify-end">
        <div role="alert" class="alert alert-error rounded-t-none">
          <Icon
            name="x"
            class="h-6 w-6 shrink-0 stroke-current"
          />
          <span>Scenario is marked as broken!</span>
        </div>
      </div>
    {/if}
  </div>
</div>

<FileContents
  filePath={filePath}
  scenarioName={scenarioName}
/>
