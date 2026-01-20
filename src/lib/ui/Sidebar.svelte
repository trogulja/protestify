<script lang="ts">
  import { page } from '$app/stores';

  type NavItem = {
    href: string;
    label: string;
    icon: 'dashboard' | 'tests' | 'organizations' | 'steps' | 'docs' | 'graphs';
  };

  const mainNav: NavItem[] = [
    { href: '/dashboard', label: 'Dashboard', icon: 'dashboard' },
    { href: '/', label: 'Tests', icon: 'tests' },
    { href: '/organizations', label: 'Organizations', icon: 'organizations' },
    { href: '/steps', label: 'Steps', icon: 'steps' },
  ];

  const secondaryNav: NavItem[] = [
    { href: '/docs', label: 'Docs', icon: 'docs' },
    { href: '/graphs', label: 'Graphs', icon: 'graphs' },
  ];

  function isActive(href: string, currentPath: string): boolean {
    if (href === '/') {
      return currentPath === '/' || currentPath.startsWith('/scenario') || currentPath.startsWith('/feature') || currentPath.startsWith('/owner') || currentPath.startsWith('/team');
    }
    return currentPath.startsWith(href);
  }
</script>

<aside class="w-56 min-h-full sidebar flex flex-col">
  <nav class="flex-1 p-3">
    <ul class="space-y-1">
      {#each mainNav as item}
        <li>
          <a
            href={item.href}
            class="flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium transition-colors
              {isActive(item.href, $page.url.pathname)
                ? 'bg-primary/10 text-primary'
                : 'text-base-content/70 hover:bg-base-content/5 hover:text-base-content'}"
          >
            <span class="w-5 h-5 flex items-center justify-center opacity-70">
              {#if item.icon === 'dashboard'}
                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="7" height="9" x="3" y="3" rx="1"/><rect width="7" height="5" x="14" y="3" rx="1"/><rect width="7" height="9" x="14" y="12" rx="1"/><rect width="7" height="5" x="3" y="16" rx="1"/></svg>
              {:else if item.icon === 'tests'}
                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"/><path d="M15 2H9a1 1 0 0 0-1 1v2a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1V3a1 1 0 0 0-1-1z"/><path d="m9 14 2 2 4-4"/></svg>
              {:else if item.icon === 'organizations'}
                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M6 22V4a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v18Z"/><path d="M6 12H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h2"/><path d="M18 9h2a2 2 0 0 1 2 2v9a2 2 0 0 1-2 2h-2"/><path d="M10 6h4"/><path d="M10 10h4"/><path d="M10 14h4"/><path d="M10 18h4"/></svg>
              {:else if item.icon === 'steps'}
                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10"/><path d="m9 12 2 2 4-4"/></svg>
              {:else if item.icon === 'docs'}
                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20"/></svg>
              {:else if item.icon === 'graphs'}
                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 3v16a2 2 0 0 0 2 2h16"/><path d="M7 16l4-8 4 5 4-8"/></svg>
              {/if}
            </span>
            {item.label}
          </a>
        </li>
      {/each}
    </ul>

    <div class="mt-6 mb-2 px-3">
      <span class="text-xs font-semibold uppercase tracking-wider text-base-content/40">Resources</span>
    </div>

    <ul class="space-y-1">
      {#each secondaryNav as item}
        <li>
          <a
            href={item.href}
            class="flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium transition-colors
              {isActive(item.href, $page.url.pathname)
                ? 'bg-primary/10 text-primary'
                : 'text-base-content/70 hover:bg-base-content/5 hover:text-base-content'}"
          >
            <span class="w-5 h-5 flex items-center justify-center opacity-70">
              {#if item.icon === 'docs'}
                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20"/></svg>
              {:else if item.icon === 'graphs'}
                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 3v16a2 2 0 0 0 2 2h16"/><path d="M7 16l4-8 4 5 4-8"/></svg>
              {/if}
            </span>
            {item.label}
          </a>
        </li>
      {/each}
    </ul>
  </nav>

  <div class="p-3 border-t border-base-300">
    <a
      href="/settings"
      class="flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium transition-colors
        {$page.url.pathname === '/settings'
          ? 'bg-primary/10 text-primary'
          : 'text-base-content/70 hover:bg-base-content/5 hover:text-base-content'}"
    >
      <span class="w-5 h-5 flex items-center justify-center opacity-70">
        <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/></svg>
      </span>
      Settings
    </a>
  </div>
</aside>
