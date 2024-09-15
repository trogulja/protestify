<script lang="ts">
  import Breadcrumbs from '$lib/ui/Breadcrumbs.svelte';
  import type Owner from '$lib/model/owner';

  export let data: { owner: Owner };
  const owner = data.owner;
  const team = owner.firstTeam;
</script>

<Breadcrumbs
  name={owner.name}
  type="person"
  parent={team.name}
  parentUrl="/team/{team.name}"
/>

<h1>Owner: {owner.name}</h1>
