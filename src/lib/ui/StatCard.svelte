<script lang="ts">
  type StatCardProps = {
    title: string;
    value: number | string;
    description?: string;
    variant?: 'default' | 'success' | 'warning' | 'error' | 'info';
    href?: string;
  };

  let { title, value, description, variant = 'default', href }: StatCardProps = $props();

  const variantClasses: Record<string, string> = {
    default: 'bg-base-100',
    success: 'bg-success/10 border-success/20',
    warning: 'bg-warning/10 border-warning/20',
    error: 'bg-error/10 border-error/20',
    info: 'bg-info/10 border-info/20',
  };

  const valueClasses: Record<string, string> = {
    default: 'text-base-content',
    success: 'text-success',
    warning: 'text-warning',
    error: 'text-error',
    info: 'text-info',
  };
</script>

{#if href}
  <a
    {href}
    class="card border shadow-sm hover:shadow-md transition-shadow {variantClasses[variant]}"
  >
    <div class="card-body p-4">
      <h3 class="card-title text-sm font-medium opacity-70">{title}</h3>
      <p class="text-3xl font-bold {valueClasses[variant]}">{value}</p>
      {#if description}
        <p class="text-xs opacity-60">{description}</p>
      {/if}
    </div>
  </a>
{:else}
  <div class="card border shadow-sm {variantClasses[variant]}">
    <div class="card-body p-4">
      <h3 class="card-title text-sm font-medium opacity-70">{title}</h3>
      <p class="text-3xl font-bold {valueClasses[variant]}">{value}</p>
      {#if description}
        <p class="text-xs opacity-60">{description}</p>
      {/if}
    </div>
  </div>
{/if}
