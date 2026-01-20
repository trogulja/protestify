<script lang="ts">
  import settings, { type Theme } from '$lib/store/settings';

  let currentTheme = $state<Theme>('system');

  // Subscribe to settings changes
  settings.subscribe((s) => {
    currentTheme = s.theme;
  });

  function getEffectiveTheme(theme: Theme): 'light' | 'dark' {
    if (theme === 'system') {
      return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
    }
    return theme;
  }

  function applyTheme(theme: Theme) {
    const effective = getEffectiveTheme(theme);
    document.documentElement.setAttribute('data-theme', effective);
  }

  function toggleTheme() {
    const effectiveTheme = getEffectiveTheme(currentTheme);
    const newTheme: Theme = effectiveTheme === 'light' ? 'dark' : 'light';

    // Update settings (this persists the choice)
    settings.update({
      ...$settings,
      theme: newTheme,
    });

    applyTheme(newTheme);
  }

  // Initialize theme on component mount
  $effect(() => {
    applyTheme(currentTheme);

    // Listen for system theme changes if using system preference
    if (currentTheme === 'system') {
      const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
      const handler = () => applyTheme('system');
      mediaQuery.addEventListener('change', handler);
      return () => mediaQuery.removeEventListener('change', handler);
    }
  });

  let isDark = $derived(getEffectiveTheme(currentTheme) === 'dark');
</script>

<button
  onclick={toggleTheme}
  class="btn btn-ghost btn-sm btn-square"
  title={isDark ? 'Switch to light mode' : 'Switch to dark mode'}
>
  {#if isDark}
    <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <circle cx="12" cy="12" r="4"/>
      <path d="M12 2v2"/>
      <path d="M12 20v2"/>
      <path d="m4.93 4.93 1.41 1.41"/>
      <path d="m17.66 17.66 1.41 1.41"/>
      <path d="M2 12h2"/>
      <path d="M20 12h2"/>
      <path d="m6.34 17.66-1.41 1.41"/>
      <path d="m19.07 4.93-1.41 1.41"/>
    </svg>
  {:else}
    <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"/>
    </svg>
  {/if}
</button>
