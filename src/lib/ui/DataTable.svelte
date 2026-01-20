<script lang="ts" generics="T extends Record<string, unknown>">
  import type { Snippet } from 'svelte';

  type Column<T> = {
    key: keyof T;
    label: string;
    sortable?: boolean;
    align?: 'left' | 'center' | 'right';
    class?: string;
  };

  type Props = {
    data: T[];
    columns: Column<T>[];
    rowClass?: (item: T) => string;
    onRowClick?: (item: T) => void;
    emptyMessage?: string;
    cell?: Snippet<[{ item: T; column: Column<T>; value: unknown }]>;
  };

  let {
    data,
    columns,
    rowClass,
    onRowClick,
    emptyMessage = 'No data available',
    cell,
  }: Props = $props();

  let sortKey = $state<keyof T | null>(null);
  let sortDirection = $state<'asc' | 'desc'>('asc');

  function handleSort(column: Column<T>) {
    if (!column.sortable) return;

    if (sortKey === column.key) {
      sortDirection = sortDirection === 'asc' ? 'desc' : 'asc';
    } else {
      sortKey = column.key;
      sortDirection = 'asc';
    }
  }

  let sortedData = $derived.by(() => {
    if (!sortKey) return data;

    return [...data].sort((a, b) => {
      const aVal = a[sortKey as keyof T];
      const bVal = b[sortKey as keyof T];

      if (aVal === bVal) return 0;
      if (aVal === null || aVal === undefined) return 1;
      if (bVal === null || bVal === undefined) return -1;

      const comparison = aVal < bVal ? -1 : 1;
      return sortDirection === 'asc' ? comparison : -comparison;
    });
  });

  function getAlignClass(align?: 'left' | 'center' | 'right'): string {
    switch (align) {
      case 'center': return 'text-center';
      case 'right': return 'text-right';
      default: return 'text-left';
    }
  }
</script>

<div class="card bg-base-100 border shadow-sm">
  <div class="overflow-x-auto">
    <table class="table table-sm">
      <thead>
        <tr>
          {#each columns as column}
            <th
              class="{getAlignClass(column.align)} {column.class ?? ''} {column.sortable ? 'cursor-pointer hover:bg-base-200' : ''}"
              onclick={() => handleSort(column)}
            >
              <div class="flex items-center gap-1 {column.align === 'right' ? 'justify-end' : column.align === 'center' ? 'justify-center' : ''}">
                {column.label}
                {#if column.sortable}
                  {#if sortKey === column.key}
                    {#if sortDirection === 'asc'}
                      <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24"><path fill="currentColor" d="m7 14l5-5l5 5z"/></svg>
                    {:else}
                      <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24"><path fill="currentColor" d="m7 10l5 5l5-5z"/></svg>
                    {/if}
                  {:else}
                    <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24" class="opacity-30"><path fill="currentColor" d="M7 10l5 5l5-5z"/></svg>
                  {/if}
                {/if}
              </div>
            </th>
          {/each}
        </tr>
      </thead>
      <tbody>
        {#if sortedData.length === 0}
          <tr>
            <td colspan={columns.length} class="text-center opacity-60 py-8">
              {emptyMessage}
            </td>
          </tr>
        {:else}
          {#each sortedData as item}
            <tr
              class="{rowClass?.(item) ?? ''} {onRowClick ? 'cursor-pointer hover:bg-base-200' : ''}"
              onclick={() => onRowClick?.(item)}
            >
              {#each columns as column}
                <td class="{getAlignClass(column.align)} {column.class ?? ''}">
                  {#if cell}
                    {@render cell({ item, column, value: item[column.key] })}
                  {:else}
                    {item[column.key]}
                  {/if}
                </td>
              {/each}
            </tr>
          {/each}
        {/if}
      </tbody>
    </table>
  </div>
</div>
