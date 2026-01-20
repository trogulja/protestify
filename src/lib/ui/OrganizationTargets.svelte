<script lang="ts">
  import { open } from '@tauri-apps/plugin-shell';
  import { toSentenceCase } from '$lib/utils';
  import settings from '$lib/store/settings';

  import type {CommandCollection, OpenCommand} from '$lib/ui/CommandPreview.svelte';

  interface Props {
    command?: CommandCollection;
    targets?: string[];
    users?: Record<string, string>;
    slug?: string;
  }

  let {
    command = $bindable(),
    targets = [],
    users = {},
    slug = ''
  }: Props = $props();

  const {e2ePwd, e2eUrl} = $settings;

  let possibleUsers = $derived(Object.keys(users));
  let firstUser = $derived(possibleUsers?.[0] ?? '');
  let selectedUser = $state('');

  // Initialize selectedUser when users change
  $effect(() => {
    if (firstUser && !selectedUser) {
      selectedUser = firstUser;
    }
  });

  let isUserSelectOpen = $state(false);

  type UserTargets = {
    name: string;
    class: string;
    url: string;
    command: OpenCommand;
  }[];

  let userTargets: UserTargets = $state([]);

  function createUrl(baseUrl: string, email: string, target?: {flag?: string, date?: string}) {
    const url = new URL(baseUrl);
    url.searchParams.append('dat', `${email}:${e2ePwd}`);
    if (target?.flag) url.searchParams.append('flag', target.flag);
    if (target?.date) url.searchParams.append('mockDate', target.date);
    return url.href;
  }

  function getUserTargets(user: string, email: string) {
    const ut: UserTargets = targets.map((target) => {
      const [name, url, flag, date] = target.split(';');
      const baseUrl = `${e2eUrl}/${slug}${url}`;
      return {
        name: toSentenceCase(name),
        class: 'btn-info',
        url: createUrl(baseUrl, email, {flag, date}),
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
      url: createUrl(`${e2eUrl}/launchpad`, email),
      command: {
        url: slug,
        screen: 'Launchpad',
        user,
      },
    })

    userTargets = ut;
  }


  $effect(() => {
    if (targets.length > 0) {
      getUserTargets(selectedUser, users[selectedUser]);
    }
  })

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
        <li><button onclick={() => selectUser(user)}>{user}</button></li>
      {/each}
    </ul>
  </details>
</div>

<div class="flex flex-wrap gap-2 ml-6">
  {#each userTargets as target}
    <button
      class="btn {target.class}"
      onmouseenter={() => showCodeHint(target.command)}
      onmouseleave={hideCodeHint}
      onclick={() => openLink(target.url)}
    >
      {target.name}
    </button>
  {/each}
</div>
