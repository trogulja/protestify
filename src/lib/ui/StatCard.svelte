<script lang="ts">
  type StatCardProps = {
    title: string;
    value: number | string;
    subtitle?: string;
    variant?: 'default' | 'success' | 'warning' | 'error' | 'info';
    href?: string;
  };

  let { title, value, subtitle, variant = 'default', href }: StatCardProps = $props();
</script>

{#if href}
  <a
    href={href}
    class="stat-card block cursor-pointer"
    class:stat-card-error={variant === 'error'}
    class:stat-card-success={variant === 'success'}
  >
    <span class="stat-card-label">{title}</span>
    <div class="stat-card-value mt-2" class:text-error={variant === 'error'} class:text-success={variant === 'success'}>{value}</div>
    {#if subtitle}
      <p class="text-xs text-base-content/50">{subtitle}</p>
    {/if}
  </a>
{:else}
  <div
    class="stat-card"
    class:stat-card-error={variant === 'error'}
    class:stat-card-success={variant === 'success'}
  >
    <span class="stat-card-label">{title}</span>
    <div class="stat-card-value mt-2" class:text-error={variant === 'error'} class:text-success={variant === 'success'}>{value}</div>
    {#if subtitle}
      <p class="text-xs text-base-content/50">{subtitle}</p>
    {/if}
  </div>
{/if}
