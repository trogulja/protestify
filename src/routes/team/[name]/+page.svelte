<script lang="ts">
  import Breadcrumbs from '$lib/ui/Breadcrumbs.svelte';
  import type Team from '$lib/model/team';

  export let data: { team: Team };
  const team = data.team;
</script>

<Breadcrumbs
  name={team.name}
  type="team"
/>

<h1>Team: {team.name}</h1>
