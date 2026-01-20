<script lang="ts">
  type Tab = {
    id: string;
    label: string;
    icon: string;
  };

  let { tabs, activeTab = $bindable() }: { tabs: Tab[]; activeTab: string } = $props();
</script>

<div role="tablist" class="tabs tabs-bordered tabs-lg mb-6">
  {#each tabs as tab}
    <button
      role="tab"
      class="tab gap-2 {activeTab === tab.id ? 'tab-active' : ''}"
      onclick={() => (activeTab = tab.id)}
    >
      {#if tab.icon === 'folder'}
        <svg xmlns="http://www.w3.org/2000/svg" width="1.2em" height="1.2em" viewBox="0 0 24 24"><path fill="currentColor" d="M4 20q-.825 0-1.412-.587T2 18V6q0-.825.588-1.412T4 4h6l2 2h8q.825 0 1.413.588T22 8v10q0 .825-.587 1.413T20 20z"/></svg>
      {:else if tab.icon === 'key'}
        <svg xmlns="http://www.w3.org/2000/svg" width="1.2em" height="1.2em" viewBox="0 0 24 24"><path fill="currentColor" d="M7 14q-.825 0-1.412-.587T5 12t.588-1.412T7 10t1.413.588T9 12t-.587 1.413T7 14m0 4q-2.5 0-4.25-1.75T1 12t1.75-4.25T7 6q1.675 0 3.038.825T12.2 9H21l3 3l-4.5 4.5l-2-1.5l-2 1.5l-2.125-1.5H12.2q-.8 1.35-2.162 2.175T7 18"/></svg>
      {:else if tab.icon === 'settings'}
        <svg xmlns="http://www.w3.org/2000/svg" width="1.2em" height="1.2em" viewBox="0 0 24 24"><path fill="currentColor" d="m10.135 21l-.362-2.892q-.479-.145-1.035-.454q-.557-.31-.947-.664l-2.668 1.135l-1.865-3.25l2.306-1.739q-.045-.27-.073-.558q-.03-.288-.03-.559q0-.252.03-.53q.028-.278.073-.626L3.258 9.126l1.865-3.212L7.771 7.03q.448-.373.97-.673q.52-.3 1.013-.464L10.134 3h3.732l.361 2.912q.575.202 1.016.463t.909.654l2.725-1.115l1.865 3.211l-2.382 1.796q.082.31.092.569t.01.51q0 .233-.02.491q-.019.259-.088.626l2.344 1.758l-1.865 3.25l-2.681-1.154q-.467.393-.94.673t-.985.445L13.866 21zm1.838-6.5q1.046 0 1.773-.727T14.473 12t-.727-1.773t-1.773-.727q-1.052 0-1.776.727T9.473 12t.724 1.773t1.776.727"/></svg>
      {/if}
      {tab.label}
    </button>
  {/each}
</div>
