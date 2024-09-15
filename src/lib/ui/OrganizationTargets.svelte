<script lang="ts">
  import { open } from '@tauri-apps/plugin-shell';
  import { toSentenceCase } from '$lib/utils';
  import settings from '$lib/store/settings';

  import type {CommandCollection, OpenCommand} from '$lib/ui/CommandPreview.svelte';

  export let command: CommandCollection = {isEmptyCommand: true};
  export let targets: string[] = [];
  export let users: Record<string, string> = {};
  export let slug: string = '';
  let e2ePwd = $settings.e2ePwd;
  let e2eUrl = $settings.e2eUrl;

  let isUserSelectOpen = false;
  let possibleUsers: string[] = [];
  let selectedUser = '';

  type UserTargets = {
    name: string;
    class: string;
    url: string;
    command: OpenCommand;
  }[];

  let userTargets: UserTargets = [];

  function createUrl(baseUrl: string, email: string, password: string, target?: {flag?: string, date?: string}) {
    const url = new URL(baseUrl);
    url.searchParams.append('dat', `${email}:${password}`);
    if (target?.flag) url.searchParams.append('flag', target.flag);
    if (target?.date) url.searchParams.append('mockDate', target.date);
    return url.href;
  }

  function getUserTargets(targets: string[], slug: string, user: string, email: string, password: string, e2eUrl: string) {
    const ut: UserTargets = targets.map((target) => {
      const [name, url, flag, date] = target.split(';');
      const baseUrl = `${e2eUrl}/${slug}${url}`;
      return {
        name: toSentenceCase(name),
        class: 'btn-info',
        url: createUrl(baseUrl, email, password, {flag, date}),
        command: {
          url: slug,
          screen: toSentenceCase(name),
          user,
          flag: flag || '',
          date: date || '',
        },
      };
    });

    ut.push({
      name: 'Launchpad',
      class: 'btn-primary',
      url: createUrl(`${e2eUrl}/launchpad`, email, password),
      command: {
        url: slug,
        screen: 'Launchpad',
        user,
      },
    })

    return ut;
  }

  $: if (Object.keys(users).length > 0) {
    possibleUsers = Object.keys(users);
    selectedUser = possibleUsers[0];
  }

  $: if (targets.length > 0) {
    userTargets = getUserTargets(targets, slug, selectedUser, users[selectedUser], e2ePwd, e2eUrl);
  }

  function selectUser(name: string) {
    selectedUser = name;
    isUserSelectOpen = false;
  }

  function showCodeHint(cmd: OpenCommand) {
    command = cmd;
  }

  function hideCodeHint() {
    command = {isEmptyCommand: true};
  }

  async function openLink(url: string) {
    await open(url);
  }
</script>

<div>Organization targets</div>
<div>
  <details class="dropdown" bind:open={isUserSelectOpen}>
    <summary class="btn btn-outline btn-sm m-1">{selectedUser}</summary>
    <ul class="menu dropdown-content bg-base-100 rounded-box z-[1] w-52 p-2 shadow">
      {#each possibleUsers as user}
        <li><button on:click={() => selectUser(user)}>{user}</button></li>
      {/each}
    </ul>
  </details>
</div>

<div class="flex flex-wrap gap-2 ml-6">
  {#each userTargets as target}
    <button
      class="btn {target.class}"
      on:mouseenter={() => showCodeHint(target.command)}
      on:mouseleave={hideCodeHint}
      on:click={() => openLink(target.url)}
    >
      {target.name}
    </button>
  {/each}
</div>
