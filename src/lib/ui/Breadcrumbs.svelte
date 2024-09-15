<script context="module" lang="ts">
  export type BCType =
    | 'table'
    | 'feature'
    | 'scenario'
    | 'organization'
    | 'team'
    | 'person';
</script>

<script lang="ts">
  import BreadcrumbsItem from '$lib/ui/BreadcrumbsItem.svelte';

  export let parent: string = '';
  export let parentUrl: string | null = null;

  export let name: string;
  export let type: BCType;

  const typeToParentTypeMap = {
    scenario: 'feature',
    person: 'team',
  } as const;

  const isTypeWithParent = (type: BCType): type is keyof typeof typeToParentTypeMap => type in typeToParentTypeMap;
  const getParentType = (type: BCType) => isTypeWithParent(type) ? typeToParentTypeMap[type] : null;
  const parentType = getParentType(type);
</script>

<div class="breadcrumbs text-sm mb-4 ml-4">
  <ul>
    <BreadcrumbsItem
      link="/"
      name="Table"
      type="table"
    />

    {#if parentType}
      <BreadcrumbsItem
        link={parentUrl}
        name={parent}
        type={parentType}
      />
    {/if}

    <BreadcrumbsItem
      name={name}
      type={type}
    />
  </ul>
</div>
