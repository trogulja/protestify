<script lang="ts">
  import "../app.css";
  import { onMount } from 'svelte';
  import { check, type Update } from '@tauri-apps/plugin-updater';
  import isUpdateAvailable from '$lib/store/is-update-available';

  import Toasts from '$lib/ui/Toasts.svelte';
  import Sidebar from '$lib/ui/Sidebar.svelte';
  import CheckForUpdatesButton from '$lib/ui/CheckForUpdatesButton.svelte';

  let { children } = $props();
  let pendingUpdate = $state<Update | null>(null);

  onMount(async () => {
    try {
      const update = await check();
      if (update?.available) {
        pendingUpdate = update;
        isUpdateAvailable.set(true);
      }
    } catch (error) {
      console.error('Failed to check for updates:', error);
    }
  });
</script>

<div class="flex h-screen flex-col">
  <header class="navbar bg-base-300 border-b border-base-300 shrink-0">
    <div class="flex-1">
      <a href="/dashboard" class="btn btn-ghost text-xl">
        <img src="/kraken.svg" alt="protestify" class="w-10 h-10" />
        proTestify
      </a>
    </div>
    {#if $isUpdateAvailable && pendingUpdate}
      <div class="flex-none mr-4">
        <CheckForUpdatesButton update={pendingUpdate} />
      </div>
    {/if}
    <div class="flex-none mr-4">
      <span class="text-sm opacity-50">v1.0.0</span>
    </div>
  </header>

  <div class="flex flex-1 overflow-hidden">
    <Sidebar />

    <main class="flex-1 overflow-auto">
      <div class="container mx-auto p-4">
        <Toasts />
        {@render children()}
      </div>
    </main>
  </div>
</div>
