<script lang="ts">
  import type {CommandCollection} from '$lib/ui/CommandPreview.svelte';

  import Breadcrumbs from '$lib/ui/Breadcrumbs.svelte';
  import Icon from '$lib/ui/Icon.svelte';
  import CommandPreview from '$lib/ui/CommandPreview.svelte';
  import FileContents from '$lib/ui/FileContents.svelte';
  import OrganizationTargets from '$lib/ui/OrganizationTargets.svelte';
  import TestRunner from '$lib/ui/TestRunner.svelte';

  let { data } = $props();

  let feature = $derived(data.feature);
  let filePath = $derived(data.filePath);
  let scenarios = $derived(data.scenarios);

  let command: CommandCollection = $state({isEmptyCommand: true});

  // @ts-expect-error - for debugging
  $effect(() => { window.k = feature; });
</script>

<Breadcrumbs
  name={feature.name}
  type="feature"
/>

<div class="flex gap-6 mb-6">
  <div class="flex-1 card bg-neutral w-96 shadow-xl">
    <div class="card-body">
      <OrganizationTargets
        bind:command={command}
        targets={feature.targets}
        users={feature.organizationUsers}
        slug={feature.organizationSlug}
      />

      <div class="divider"></div>

      <TestRunner
        bind:command={command}
        featureFile={filePath}
      />

      <div class="divider"></div>

      <CommandPreview
        command={command}
        maxLineLength={feature.commandMaxLen}
      />
    </div>

  </div>

  <div class="flex-1 card bg-neutral w-96 shadow-xl">
    <div class="card-body">
      <h2 class="card-title text-white mb-6">{feature.name}</h2>

      <div>
        <Icon
          name="organization"
          class="h-5 w-5 mr-2 inline-block align-text-bottom"
        />
        <a href="/organization/{feature.organizationId}" class="link link-hover link-primary">
          {feature.organizationName}
        </a>
      </div>
      <div>
        <Icon
          name="person"
          class="h-5 w-5 mr-2 inline-block align-text-bottom"
        />
        <a href="/owner/{feature.organizationOwner}" class="link link-hover link-primary">
          {feature.organizationOwner}
        </a>
      </div>
      <div>
        <Icon
          name="team"
          class="h-5 w-5 mr-2 inline-block align-text-bottom"
        />
        <a href="/team/{feature.organizationTeam}" class="link link-hover link-primary">
          {feature.organizationTeam}
        </a>
      </div>

      <div class="divider"></div>

      <div>Scenarios:</div>
      {#each scenarios as scenario}
        <div>
          <Icon
            name="scenario"
            class="h-5 w-5 mr-2 inline-block align-text-bottom"
          />
          <a href="/scenario/{scenario.id}" class="link link-hover link-primary">
            {scenario.name}
          </a>
        </div>
      {/each}

      <div class="divider"></div>

      <div>
        <Icon
          name="target"
          class="h-5 w-5 mr-2 inline-block align-text-bottom"
        />
        {feature.targets.length} target screens
      </div>
      <div>
        <Icon
          name="team"
          class="h-5 w-5 mr-2 inline-block align-text-bottom"
        />
        {feature.organizationUsersCount} defined users
      </div>
      <div>
        <Icon
          name="flag"
          class="h-5 w-5 mr-2 inline-block align-text-bottom"
        />
        {feature.flags.length} flags
      </div>
      <div>
        <Icon
          name="date"
          class="h-5 w-5 mr-2 inline-block align-text-bottom"
        />
        {feature.mockDates.length} mock date
      </div>
      <div>
        <Icon
          name="tag"
          class="h-5 w-5 mr-2 inline-block align-text-bottom"
        />
        {feature.tags.length} tags {feature.tags.join(', ')}
      </div>
    </div>

    {#if feature.isBroken}
      <div class="card-actions justify-end">
        <div role="alert" class="alert alert-error rounded-t-none">
          <Icon
            name="x"
            class="h-6 w-6 shrink-0 stroke-current"
          />
          <span>Feature is marked as broken!</span>
        </div>
      </div>
    {/if}
  </div>
</div>

<FileContents
  filePath={filePath}
/>
