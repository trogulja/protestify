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

  // Compute stats
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
</script>

<div class="space-y-6">
  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-bold">Step Definitions</h1>
    <div class="badge badge-neutral">{data.steps.length} steps</div>
  </div>

  {#if data.error}
    <div class="alert alert-error">
      <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 24 24"><path fill="currentColor" d="M12 17q.425 0 .713-.288T13 16t-.288-.712T12 15t-.712.288T11 16t.288.713T12 17m-1-4h2V7h-2zm1 9q-2.075 0-3.9-.788t-3.175-2.137T2.788 15.9T2 12t.788-3.9t2.137-3.175T8.1 2.788T12 2t3.9.788t3.175 2.137T21.213 8.1T22 12t-.788 3.9t-2.137 3.175t-3.175 2.138T12 22"/></svg>
      <span>Failed to load steps: {data.error}</span>
    </div>
  {:else}
    <div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-7 gap-3">
      {#each STEP_CATEGORIES as category}
        <button
          class="btn btn-sm {filterCategory === category ? 'btn-primary' : 'btn-ghost'} justify-between"
          onclick={() => filterCategory = filterCategory === category ? '' : category}
        >
          <span>{category}</span>
          <span class="badge {CATEGORY_COLORS[category]} badge-sm">{stats.byCategory[category]}</span>
        </button>
      {/each}
    </div>

    <div class="flex flex-wrap gap-4">
      <label class="input input-bordered input-sm flex items-center gap-2 w-64">
        <input
          type="text"
          class="grow"
          placeholder="Search patterns..."
          bind:value={searchTerm}
          onkeydown={handleKeydown}
        />
        <svg xmlns="http://www.w3.org/2000/svg" width="1.2em" height="1.2em" viewBox="0 0 24 24"><path fill="currentColor" d="m19.485 20.154l-6.262-6.262q-.75.639-1.725.989t-1.96.35q-2.402 0-4.066-1.663T3.808 9.503T5.47 5.436t4.064-1.667t4.068 1.664T15.268 9.5q0 1.042-.369 2.017t-.97 1.668l6.262 6.261zM9.539 14.23q1.99 0 3.36-1.37t1.37-3.361t-1.37-3.36t-3.36-1.37t-3.361 1.37t-1.37 3.36t1.37 3.36t3.36 1.37"/></svg>
      </label>

      <select class="select select-bordered select-sm" bind:value={filterKeyword}>
        <option value="">All Keywords</option>
        <option value="Given">Given ({stats.byKeyword.Given})</option>
        <option value="When">When ({stats.byKeyword.When})</option>
        <option value="Then">Then ({stats.byKeyword.Then})</option>
        <option value="And">And ({stats.byKeyword.And})</option>
        <option value="But">But ({stats.byKeyword.But})</option>
      </select>

      <label class="label cursor-pointer gap-2">
        <input type="checkbox" class="toggle toggle-sm toggle-warning" bind:checked={showProblematicOnly} />
        <span class="label-text">
          Problematic only
          {#if stats.problematic > 0}
            <span class="badge badge-warning badge-xs">{stats.problematic}</span>
          {/if}
        </span>
      </label>
    </div>

    <div class="card bg-base-100 border shadow-sm">
      <div class="overflow-x-auto">
        <table class="table table-sm">
          <thead>
            <tr>
              <th class="w-20">Keyword</th>
              <th>Pattern</th>
              <th class="w-24">Category</th>
              <th>File</th>
              <th class="w-16">Line</th>
            </tr>
          </thead>
          <tbody>
            {#each filteredSteps as step}
              <tr class={step.is_problematic ? 'bg-warning/10' : ''}>
                <td>
                  <span class="badge badge-outline badge-sm">{step.keyword}</span>
                </td>
                <td>
                  <code class="text-xs bg-base-200 px-1 rounded">{step.pattern}</code>
                  {#if step.is_problematic && step.problem_reason}
                    <span class="badge badge-warning badge-xs ml-2" title={step.problem_reason}>!</span>
                  {/if}
                </td>
                <td>
                  <span class="badge {CATEGORY_COLORS[step.category as StepCategory]} badge-sm">{step.category}</span>
                </td>
                <td class="text-xs opacity-70 font-mono">
                  {getRelativePath(step.file_path)}
                </td>
                <td class="text-xs opacity-70 font-mono">
                  {step.line_number}
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>

    <div class="text-sm opacity-60">
      Showing {filteredSteps.length} of {data.steps.length} step definitions
    </div>
  {/if}
</div>
