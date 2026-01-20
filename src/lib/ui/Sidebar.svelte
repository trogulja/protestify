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

<aside class="w-56 min-h-full bg-base-200 text-base-content flex flex-col">
  <ul class="menu p-2 flex-1">
    {#each mainNav as item}
      <li>
        <a
          href={item.href}
          class="flex items-center gap-3 {isActive(item.href, $page.url.pathname) ? 'active' : ''}"
        >
          {#if item.icon === 'dashboard'}
            <svg xmlns="http://www.w3.org/2000/svg" width="1.25em" height="1.25em" viewBox="0 0 24 24"><path fill="currentColor" d="M13 9V3h8v6zM3 13V3h8v10zm10 8V11h8v10zM3 21v-6h8v6z"/></svg>
          {:else if item.icon === 'tests'}
            <svg xmlns="http://www.w3.org/2000/svg" width="1.25em" height="1.25em" viewBox="0 0 24 24"><path fill="currentColor" d="M4 4v16h16V4zm2 2h12v2H6zm0 4h12v2H6zm0 4h12v2H6z"/></svg>
          {:else if item.icon === 'organizations'}
            <svg xmlns="http://www.w3.org/2000/svg" width="1.25em" height="1.25em" viewBox="0 0 24 24"><path fill="currentColor" d="M6 21v-9H2l10-9l10 9h-4v9zm6-7.7l2.1-2.1l1.4 1.4L12 16.1l-3.5-3.5l1.4-1.4z"/></svg>
          {:else if item.icon === 'steps'}
            <svg xmlns="http://www.w3.org/2000/svg" width="1.25em" height="1.25em" viewBox="0 0 24 24"><path fill="currentColor" d="M12 2L4 5v6.09c0 5.05 3.41 9.76 8 10.91c4.59-1.15 8-5.86 8-10.91V5zm-1.06 13.54L7.4 12l1.41-1.41l2.12 2.12l4.24-4.24l1.41 1.41z"/></svg>
          {:else if item.icon === 'docs'}
            <svg xmlns="http://www.w3.org/2000/svg" width="1.25em" height="1.25em" viewBox="0 0 24 24"><path fill="currentColor" d="M7.616 21q-1.085 0-1.85-.766Q5 19.47 5 18.385V6q0-1.258.871-2.129T8 3h11v13.77q-.663 0-1.14.475t-.475 1.14t.476 1.139T19 20v1zm.769-5.23h1V4h-1zM7.615 20h9.364q-.285-.33-.44-.732q-.155-.4-.155-.884q0-.457.152-.87t.443-.745H7.616q-.689 0-1.152.476T6 18.385q0 .688.464 1.151T7.616 20"/></svg>
          {:else if item.icon === 'graphs'}
            <svg xmlns="http://www.w3.org/2000/svg" width="1.25em" height="1.25em" viewBox="0 0 24 24"><path fill="currentColor" d="M3 21v-2h2V9h4v10h2V5h4v14h2V11h4v10h2v2z"/></svg>
          {/if}
          {item.label}
        </a>
      </li>
    {/each}

    <li class="menu-title mt-4">
      <span class="text-xs uppercase tracking-wider opacity-60">Resources</span>
    </li>

    {#each secondaryNav as item}
      <li>
        <a
          href={item.href}
          class="flex items-center gap-3 {isActive(item.href, $page.url.pathname) ? 'active' : ''}"
        >
          {#if item.icon === 'docs'}
            <svg xmlns="http://www.w3.org/2000/svg" width="1.25em" height="1.25em" viewBox="0 0 24 24"><path fill="currentColor" d="M7.616 21q-1.085 0-1.85-.766Q5 19.47 5 18.385V6q0-1.258.871-2.129T8 3h11v13.77q-.663 0-1.14.475t-.475 1.14t.476 1.139T19 20v1zm.769-5.23h1V4h-1zM7.615 20h9.364q-.285-.33-.44-.732q-.155-.4-.155-.884q0-.457.152-.87t.443-.745H7.616q-.689 0-1.152.476T6 18.385q0 .688.464 1.151T7.616 20"/></svg>
          {:else if item.icon === 'graphs'}
            <svg xmlns="http://www.w3.org/2000/svg" width="1.25em" height="1.25em" viewBox="0 0 24 24"><path fill="currentColor" d="M3 21v-2h2V9h4v10h2V5h4v14h2V11h4v10h2v2z"/></svg>
          {/if}
          {item.label}
        </a>
      </li>
    {/each}
  </ul>

  <div class="p-4 border-t border-base-300">
    <a href="/settings" class="btn btn-ghost btn-sm w-full justify-start gap-3">
      <svg xmlns="http://www.w3.org/2000/svg" width="1.25em" height="1.25em" viewBox="0 0 24 24"><path fill="currentColor" d="m10.135 21l-.362-2.892q-.479-.145-1.035-.454q-.557-.31-.947-.664l-2.668 1.135l-1.865-3.25l2.306-1.739q-.045-.27-.073-.558q-.03-.288-.03-.559q0-.252.03-.53q.028-.278.073-.626L3.258 9.126l1.865-3.212L7.771 7.03q.448-.373.97-.673q.52-.3 1.013-.464L10.134 3h3.732l.361 2.912q.575.202 1.016.463t.909.654l2.725-1.115l1.865 3.211l-2.382 1.796q.082.31.092.569t.01.51q0 .233-.02.491q-.019.259-.088.626l2.344 1.758l-1.865 3.25l-2.681-1.154q-.467.393-.94.673t-.985.445L13.866 21zm1.838-6.5q1.046 0 1.773-.727T14.473 12t-.727-1.773t-1.773-.727q-1.052 0-1.776.727T9.473 12t.724 1.773t1.776.727"/></svg>
      Settings
    </a>
  </div>
</aside>
