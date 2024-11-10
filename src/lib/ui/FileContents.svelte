<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Icon from '$lib/ui/Icon.svelte';

  type GetFileContents = {
    ok: string;
    err: string;
  }

  const NO_FILE_CONTENTS = [['0', 'No file contents']];

  let fileContents: string[][] = $state(NO_FILE_CONTENTS);
  interface Props {
    filePath: string;
    scenarioName?: string | undefined;
  }

  let { filePath, scenarioName = undefined }: Props = $props();

  // I am on a "Template center" screen and flag "budgetTemplates,templatesTypesAndCreators" is enabled
  const patterns = [
    {pattern: /(?<!\bflag\s)("[^"]+")/g, color: 'text-green-400'},
    {pattern: /^\s*Scenario[^:]*: (.+)$/, color: 'text-info'},
    {pattern: /^\s*Feature[^:]*: (.+)$/, color: 'text-primary'},
    {pattern: /^\s*(Scenario:|Scenario outline:) .+$/, color: 'text-orange-500'},
    {pattern: /^\s*(Feature:) .+$/, color: 'text-orange-500'},
    {pattern: /^\s*(Given|Then|When|And|But|\*) .+$/, color: 'text-orange-500'},
    // {pattern: /^\s*(\| .+)$/, color: 'text-cyan-400'},
    {pattern: /^.* ([^\s]+ waits? for \d+ seconds?)$/, color: 'text-red-600'},
    {pattern: /^.* (and flag ".*" is enabled)$/, color: 'text-red-600'},
  ];

  function colorCodeLine(line: string): string {
    let output = line;

    for (const { pattern, color } of patterns) {
      output = output.replace(pattern, (match, ...groups) => {
        // Replace each group with a span element
        return match.replace(groups[0], `<span class="${color}">${groups[0]}</span>`);
      });
    }

    return output;
  }

  async function loadFile(filePath: string, scenarioName?: string) {
    if (!filePath) return NO_FILE_CONTENTS;

    const args = scenarioName ? {filePath, scenarioName} : {filePath};
    const result = await invoke<GetFileContents>('get_file_contents', args);

    if (result?.err) return [['0', `<span class="text-error">${result.err}</span>`]];

    const lines = result?.ok.trimEnd().split('\n');
    if (!lines?.length) return NO_FILE_CONTENTS;

    const contents = lines.map((line) => {
      // Escape HTML entities
      line = line.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
      const match = line.match(/^(\d+):(.*)$/);
      return match ? [match[1], colorCodeLine(match[2])] : ['0', line];
    });

    fileContents = contents;
  }

  $effect(() => {
    loadFile(filePath, scenarioName);
  })
</script>

<div class="relative">
  <button
    class="absolute z-50 top-0 right-0 btn btn-square rounded-tl-none rounded-br-none"
    onclick={() => loadFile(filePath, scenarioName)}
  >
    <Icon
      name="reload"
      class="h-6 w-6"
    />
  </button>
  <div class="mockup-code">
    {#each fileContents as line}
      <pre data-prefix={line[0]}><code>{@html line[1]}</code></pre>
    {/each}
  </div>
</div>
