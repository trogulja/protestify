<script>
  import "../app.css";
  import isUpdateAvailable from '$lib/store/is-update-available';

  import Toasts from '$lib/ui/Toasts.svelte';
  import CheckForUpdatesButton from '$lib/ui/CheckForUpdatesButton.svelte';
</script>

<div class="navbar bg-base-300">
  <div class="flex-1">
    <a href="/" class="btn btn-ghost text-xl">
      <img src="/kraken.svg" alt="protestify" class="w-10 h-10" />
      proTestify
    </a>
  </div>
  {#if $isUpdateAvailable}
    <div class="flex-none mr-6">
      <CheckForUpdatesButton />
    </div>
  {/if}
  <div class="flex-none">
    <a href="/graphs" class="btn btn-square btn-ghost">
      <svg xmlns="http://www.w3.org/2000/svg" width="2.8em" height="2.8em" viewBox="0 0 24 24"><path fill="currentColor" d="M11.058 13.808q1.142 0 1.946-.804t.804-1.946t-.804-1.947q-.804-.803-1.946-.803t-1.946.803t-.804 1.947t.803 1.946t1.947.804m5.036 3l-2.76-2.78q-.5.384-1.071.582q-.573.198-1.205.198q-1.564 0-2.657-1.093t-1.093-2.657T8.4 8.4t2.657-1.093T13.714 8.4t1.094 2.657q0 .633-.201 1.205t-.584 1.072l2.785 2.765zM5.616 20q-.691 0-1.153-.462T4 18.384V15h1v3.385q0 .23.192.423t.423.192H9v1zM15 20v-1h3.385q.23 0 .423-.192t.192-.424V15h1v3.385q0 .69-.462 1.152T18.384 20zM4 9V5.616q0-.691.463-1.153T5.616 4H9v1H5.616q-.231 0-.424.192T5 5.616V9zm15 0V5.616q0-.231-.192-.424T18.384 5H15V4h3.385q.69 0 1.152.463T20 5.616V9z"/></svg>
    </a>
  </div>
  <div class="flex-none">
    <a href="/docs" class="btn btn-square btn-ghost">
      <svg xmlns="http://www.w3.org/2000/svg" width="2.5em" height="2.5em" viewBox="0 0 24 24"><path fill="currentColor" d="M7.616 21q-1.085 0-1.85-.766Q5 19.47 5 18.385V6q0-1.258.871-2.129T8 3h11v13.77q-.663 0-1.14.475t-.475 1.14t.476 1.139T19 20v1zm.769-5.23h1V4h-1zM7.615 20h9.364q-.285-.33-.44-.732q-.155-.4-.155-.884q0-.457.152-.87t.443-.745H7.616q-.689 0-1.152.476T6 18.385q0 .688.464 1.151T7.616 20"/></svg>
    </a>
  </div>
  <div class="flex-none">
    <a href="/settings" class="btn btn-square btn-ghost">
      <svg xmlns="http://www.w3.org/2000/svg" width="2.5em" height="2.5em" viewBox="0 0 24 24"><path fill="currentColor" d="m10.135 21l-.362-2.892q-.479-.145-1.035-.454q-.557-.31-.947-.664l-2.668 1.135l-1.865-3.25l2.306-1.739q-.045-.27-.073-.558q-.03-.288-.03-.559q0-.252.03-.53q.028-.278.073-.626L3.258 9.126l1.865-3.212L7.771 7.03q.448-.373.97-.673q.52-.3 1.013-.464L10.134 3h3.732l.361 2.912q.575.202 1.016.463t.909.654l2.725-1.115l1.865 3.211l-2.382 1.796q.082.31.092.569t.01.51q0 .233-.02.491q-.019.259-.088.626l2.344 1.758l-1.865 3.25l-2.681-1.154q-.467.393-.94.673t-.985.445L13.866 21zm1.838-6.5q1.046 0 1.773-.727T14.473 12t-.727-1.773t-1.773-.727q-1.052 0-1.776.727T9.473 12t.724 1.773t1.776.727"/></svg>
    </a>
  </div>
</div>

<div class="container mx-auto py-4">
  <Toasts />
  <slot />
</div>
