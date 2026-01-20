<script lang="ts">
  import { STEP_CATEGORIES, CATEGORY_COLORS, type StepCategory, type StepDefinition } from '$lib/model/step';

  let { data } = $props();

  let searchTerm = $state('');
  let filterCategory = $state<string>('');
  let filterKeyword = $state<string>('');
  let showProblematicOnly = $state(false);

  let filteredSteps = $derived(
    data.steps.filter((step: StepDefinition) => {
      const matchesSearch =
        step.pattern.toLowerCase().includes(searchTerm.toLowerCase()) ||
        step.file_path.toLowerCase().includes(searchTerm.toLowerCase());
      const matchesCategory = !filterCategory || step.category === filterCategory;
      const matchesKeyword = !filterKeyword || step.keyword === filterKeyword;
      const matchesProblematic = !showProblematicOnly || step.is_problematic;
      return matchesSearch && matchesCategory && matchesKeyword && matchesProblematic;
    })
  );

  let stats = $derived({
    total: data.steps.length,
    byCategory: STEP_CATEGORIES.reduce((acc, cat) => {
      acc[cat] = data.steps.filter((s: StepDefinition) => s.category === cat).length;
      return acc;
    }, {} as Record<StepCategory, number>),
    byKeyword: {
      Given: data.steps.filter((s: StepDefinition) => s.keyword === 'Given').length,
      When: data.steps.filter((s: StepDefinition) => s.keyword === 'When').length,
      Then: data.steps.filter((s: StepDefinition) => s.keyword === 'Then').length,
      And: data.steps.filter((s: StepDefinition) => s.keyword === 'And').length,
      But: data.steps.filter((s: StepDefinition) => s.keyword === 'But').length,
    },
    problematic: data.steps.filter((s: StepDefinition) => s.is_problematic).length,
  });

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      searchTerm = '';
    }
  }

  function getRelativePath(fullPath: string): string {
    const parts = fullPath.split('step_definitions/');
    return parts.length > 1 ? parts[1] : fullPath;
  }

  const categoryBgColors: Record<StepCategory, string> = {
    'Navigation': 'bg-primary/15 text-primary',
    'Waits': 'bg-warning/15 text-warning',
    'Assertions': 'bg-success/15 text-success',
    'Data Setup': 'bg-info/15 text-info',
    'Flags': 'bg-secondary/15 text-secondary',
    'Actions': 'bg-accent/15 text-accent',
    'Other': 'bg-base-content/10 text-base-content/70',
  };
</script>

<div class="space-y-6">
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-xl font-semibold text-base-content mb-1">Step Definitions</h1>
      <p class="text-sm text-base-content/60">Browse and search cucumber step definitions</p>
    </div>
    <span class="text-sm text-base-content/50 font-medium">{data.steps.length} steps</span>
  </div>

  {#if data.error}
    <div class="flex items-center gap-2 px-4 py-3 rounded-lg bg-error/10 border border-error/20">
      <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 text-error" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="m15 9-6 6"/><path d="m9 9 6 6"/></svg>
      <span class="text-sm">Failed to load steps: {data.error}</span>
    </div>
  {:else}
    <div class="flex flex-wrap gap-2">
      {#each STEP_CATEGORIES as category}
        <button
          class="inline-flex items-center gap-2 px-3 py-1.5 rounded-lg text-sm font-medium transition-colors
            {filterCategory === category
              ? categoryBgColors[category]
              : 'bg-base-content/5 text-base-content/60 hover:bg-base-content/10'}"
          onclick={() => filterCategory = filterCategory === category ? '' : category}
        >
          {category}
          <span class="text-xs opacity-70">{stats.byCategory[category]}</span>
        </button>
      {/each}
    </div>

    <div class="flex flex-wrap items-center gap-3">
      <div class="relative">
        <input
          type="text"
          class="input input-sm w-64 pl-9"
          placeholder="Search patterns..."
          bind:value={searchTerm}
          onkeydown={handleKeydown}
        />
        <svg xmlns="http://www.w3.org/2000/svg" class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-base-content/40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/></svg>
      </div>

      <select class="select select-sm" bind:value={filterKeyword}>
        <option value="">All Keywords</option>
        <option value="Given">Given ({stats.byKeyword.Given})</option>
        <option value="When">When ({stats.byKeyword.When})</option>
        <option value="Then">Then ({stats.byKeyword.Then})</option>
        <option value="And">And ({stats.byKeyword.And})</option>
        <option value="But">But ({stats.byKeyword.But})</option>
      </select>

      <label class="flex items-center gap-2 cursor-pointer">
        <input type="checkbox" class="toggle toggle-sm toggle-warning" bind:checked={showProblematicOnly} />
        <span class="text-sm text-base-content/70">
          Problematic only
          {#if stats.problematic > 0}
            <span class="inline-flex items-center justify-center min-w-[1.25rem] px-1 py-0.5 rounded-full text-xs font-medium bg-warning/15 text-warning ml-1">{stats.problematic}</span>
          {/if}
        </span>
      </label>
    </div>

    <div class="card-clean overflow-hidden">
      <div class="overflow-x-auto">
        <table class="table table-sm">
          <thead>
            <tr>
              <th class="w-20">Keyword</th>
              <th>Pattern</th>
              <th class="w-28">Category</th>
              <th>File</th>
              <th class="w-16 text-right">Line</th>
            </tr>
          </thead>
          <tbody>
            {#each filteredSteps as step}
              <tr class={step.is_problematic ? 'bg-warning/5' : ''}>
                <td>
                  <span class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-base-content/10">{step.keyword}</span>
                </td>
                <td>
                  <code class="text-xs bg-base-200 px-1.5 py-0.5 rounded font-mono">{step.pattern}</code>
                  {#if step.is_problematic && step.problem_reason}
                    <span class="inline-flex items-center justify-center w-4 h-4 rounded-full bg-warning/15 text-warning text-xs ml-2" title={step.problem_reason}>!</span>
                  {/if}
                </td>
                <td>
                  <span class="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium {categoryBgColors[step.category as StepCategory]}">{step.category}</span>
                </td>
                <td class="text-xs text-base-content/60 font-mono">
                  {getRelativePath(step.file_path)}
                </td>
                <td class="text-xs text-base-content/60 font-mono text-right">
                  {step.line_number}
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>

    <p class="text-sm text-base-content/50">
      Showing {filteredSteps.length} of {data.steps.length} step definitions
    </p>
  {/if}
</div>
