<script module lang="ts">
  export type OpenCommand = {
    url: string;
    screen: string;
    user: string;
    flag?: string;
    date?: string;
  }

  export type TestCommand = {
    scenarioName?: string;
    featureFile: string;
  }

  export type EmptyCommand = {
    isEmptyCommand: true;
  }

  export type CommandCollection = OpenCommand | TestCommand | EmptyCommand;
</script>

<script lang="ts">
  const isTestCommand = (obj: CommandCollection): obj is TestCommand => 'featureFile' in obj;
  const isEmptyCommand = (obj: CommandCollection): obj is EmptyCommand => 'isEmptyCommand' in obj;

  interface Props {
    command: CommandCollection;
    maxLines?: number;
    maxLineLength?: number;
  }

  let { command, maxLines = 5, maxLineLength = 10 }: Props = $props();

  function getLines(cmd: CommandCollection): string[] {
    const lines: string[] = [];

    if (isTestCommand(cmd)) {
      const line1 = 'npx cucumber-js'.padEnd(maxLineLength, ' ');
      const line2 = cmd.featureFile.split('/features/')[1];
      lines.push(...[
        `<pre data-prefix="$"><code>${line1}</code></pre>`,
        `<pre data-prefix=""><code>  <span class="text-primary">.../${line2}</span></code></pre>`,
        ...cmd.scenarioName ? [`<pre data-prefix=""><code>  --name=<span class="text-info">${cmd.scenarioName}</span></code></pre>`] : [],
        `<pre data-prefix=""><code>  -p local</code></pre>`,
      ]);
    } else if (isEmptyCommand(cmd)) {
      lines.push(`<pre data-prefix="$"><code>${''.padEnd(maxLineLength, ' ')}</code></pre>`);
    } else {
      const { url, screen, user, flag, date } = cmd;
      lines.push(...[
        `<pre data-prefix="$"><code>open <span class="text-primary">${url}</span>${''.padEnd(maxLineLength - url.length - 5, ' ')}</code></pre>`,
        `<pre data-prefix=""><code>  --target-screen=<span class="text-info">${screen}</span></code></pre>`,
        `<pre data-prefix=""><code>  --target-user=<span class="text-info">${user}</span></code></pre>`,
        ...flag ? [`<pre data-prefix=""><code>  --flags=<span class="text-warning">${flag}</span></code></pre>`] : [],
        ...date ? [`<pre data-prefix=""><code>  --mock-date=<span class="text-warning">${date}</span></code></pre>`] : [],
      ].filter(Boolean));
    }

    for (let i = lines.length; i < maxLines; i++) {
      lines.push('<pre data-prefix=""><code></code></pre>');
    }

    return lines;
  };

  let lines = $derived(getLines(command));
</script>

<div>Command preview:</div>
<div class="mockup-code text-xs bg-base-100 ml-6">
  {#each lines as line}
    {@html line}
  {/each}
</div>
