<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import settings from '$lib/store/settings';

  import type {CommandCollection} from '$lib/ui/CommandPreview.svelte';

  export let command: CommandCollection = {isEmptyCommand: true};
  export let scenarioName: string | undefined = undefined;
  export let featureFile: string;
  let folderPath = $settings.basePath;

  async function runScenario() {
    const result = await invoke<string>('run_e2e', {folderPath, featureFile, scenarioName});
    console.log(result);
  }

  async function runFeature() {
    const result = await invoke<string>('run_e2e', {folderPath, featureFile});
    console.log(result);
  }

  function showScenarioCodeHint() {
    command = {featureFile, scenarioName};
  }

  function showFeatureCodeHint() {
    command = {featureFile};
  }

  function hideCodeHint() {
    command = {isEmptyCommand: true};
  }

  // command to run in the e2e repo is:
  // - npx cucumber-js /path/to/feature/file.feature --name "Scenario name if we want to run only one scenario" -p local
</script>

<div>Test runner</div>
<div class="flex flex-wrap gap-2 ml-6">
  {#if scenarioName}
    <button
      class="btn btn-info"
      on:mouseenter={showScenarioCodeHint}
      on:mouseleave={hideCodeHint}
      on:click={runScenario}
    >
      Run scenario
    </button>
  {/if}
  <button
    class="btn btn-primary"
    on:mouseenter={showFeatureCodeHint}
    on:mouseleave={hideCodeHint}
    on:click={runFeature}
  >
    Run feature
  </button>
</div>
