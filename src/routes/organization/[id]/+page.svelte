<script lang="ts">
  import Breadcrumbs from '$lib/ui/Breadcrumbs.svelte';

  let { data } = $props();

  const organization = data.organization;
</script>

<Breadcrumbs
  name={`${organization.id} - ${organization.name}`}
  type="organization"
/>

<div>
  Ovo je neka odabrana organizacija: "{organization.id} - {organization.name}"
</div>
