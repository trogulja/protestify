<script lang="ts">
  import { onMount } from 'svelte';

  let tooltip: HTMLElement;
  let parent: HTMLElement | null | undefined;
  let isVisible = false;
  export let grandparent: boolean = false;

  function moveTooltip(e: MouseEvent) {
    const screenLeft = e.pageX - window.scrollX + 10;
    const screenTop = e.pageY - window.scrollY + 10;

    tooltip.style.left =
      screenLeft + tooltip.clientWidth + 10 < window.innerHeight
        ? `${screenLeft}px`
        : `${screenLeft + 5 - tooltip.clientWidth}px`;

    tooltip.style.top =
      screenTop + tooltip.clientHeight + 10 < window.innerHeight
        ? `${screenTop}px`
        : `${screenTop + 5 - tooltip.clientHeight}px`;
  }

  function showTooltip(_e: Event) {
    isVisible = true;
  }

  function hideTooltip(_e: Event) {
    isVisible = false;
  }

  onMount(() => {
    if (grandparent) {
      parent = tooltip.parentElement?.parentElement;
    } else {
      parent = tooltip.parentElement;
    }
    parent?.addEventListener('mousemove', moveTooltip);
    parent?.addEventListener('mouseleave', hideTooltip);
    parent?.addEventListener('mouseenter', showTooltip);
    document.addEventListener('scroll', hideTooltip);
    return () => {
      parent?.removeEventListener('mousemove', moveTooltip);
      parent?.removeEventListener('mouseleave', hideTooltip);
      parent?.removeEventListener('mouseenter', showTooltip);
      document.removeEventListener('scroll', hideTooltip);
    };
  });
</script>

<div class="tooltip" class:show={isVisible} bind:this={tooltip}>
  <slot />
</div>

<style>
  .tooltip {
    position: fixed;
    z-index: 1000;
    visibility: hidden;
    opacity: 0;
    transition: opacity 0.3s ease, transform 0.3s ease;
    transform: translateY(-10px);
  }

  .tooltip.show {
    visibility: visible;
    opacity: 1;
    transform: translateY(0);
    transition-delay: 0.5s;
  }
</style>
