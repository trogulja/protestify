<script lang="ts">
  import Breadcrumbs from '$lib/ui/Breadcrumbs.svelte';

  let { data } = $props();
  const team = data.team;
</script>

<Breadcrumbs
  name={team.name}
  type="team"
/>

<h1>Team: {team.name}</h1>
