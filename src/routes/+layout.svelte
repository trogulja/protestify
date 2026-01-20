<script lang="ts">
  import "../app.css";
  import { onMount } from 'svelte';
  import { check, type Update } from '@tauri-apps/plugin-updater';
  import isUpdateAvailable from '$lib/store/is-update-available';
  import settings from '$lib/store/settings';

  import Toasts from '$lib/ui/Toasts.svelte';
  import Sidebar from '$lib/ui/Sidebar.svelte';
  import ThemeToggle from '$lib/ui/ThemeToggle.svelte';
  import CheckForUpdatesButton from '$lib/ui/CheckForUpdatesButton.svelte';

  let { children } = $props();
  let pendingUpdate = $state<Update | null>(null);

  onMount(async () => {
    // Initialize settings (loads persisted values)
    await settings.init();

    // Check for updates
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

<div class="flex h-screen flex-col bg-base-200">
  <header class="app-header flex items-center px-4 shrink-0">
    <div class="flex items-center gap-3 flex-1">
      <a href="/dashboard" class="flex items-center gap-2 hover:opacity-80 transition-opacity">
        <img src="/kraken.svg" alt="protestify" class="w-8 h-8" />
        <span class="font-semibold text-lg">proTestify</span>
      </a>
    </div>
    <div class="flex items-center gap-2">
      {#if $isUpdateAvailable && pendingUpdate}
        <CheckForUpdatesButton update={pendingUpdate} />
      {/if}
      <ThemeToggle />
      <span class="text-xs text-base-content/40 font-medium ml-1">v1.0.0</span>
    </div>
  </header>

  <div class="flex flex-1 overflow-hidden">
    <Sidebar />

    <main class="flex-1 overflow-auto content-area">
      <div class="p-6 max-w-7xl mx-auto">
        <Toasts />
        {@render children()}
      </div>
    </main>
  </div>
</div>
