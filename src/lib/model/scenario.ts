import type {Store} from '$lib/store';
import Model from '$lib/model';

function parseTargets(targets: string[]) {
  return targets.map((t) => {
    const [name, url, flag, date] = t.split(';');

    return {name, url, flag, date};
  });
}

export default class Scenario extends Model {
  id: string;
  name: string;
  description: string;
  steps: number;
  examples: number;
  tags: string[];
  featureId: string;
  organizationName: string
  targets: string[];
  _targets: {name: string, url: string, flag: string, date: string}[];

  constructor(store: Store, data: RustyScenario) {
    super(store);
    this.id = data.id;
    this.name = data.name;
    this.description = data.description;
    this.steps = data.steps;
    this.examples = data.examples;
    this.tags = data.tags;
    this.featureId = data.feature_id;
    this.organizationName = data.organization_name;
    this.targets = data.targets;

    this._targets = parseTargets(data.targets);
  }

  get feature() {
    return this.store.features.find((f) => f.id === this.featureId);
  }

  get featureName() {
    return this.feature?.name ?? '';
  }

  get featureFilePath() {
    return this.feature?.filePath ?? '';
  }

  get organization() {
    return this.store.organizations.find((o) => o.name === this.organizationName);
  }

  get organizationId() {
    return this.organization?.id ?? '';
  }

  get organizationSlug() {
    return `${this.organizationId}-${this.organizationName.toLowerCase().replace(/ /g, '-')}`;
  }

  get organizationOwner() {
    return this.organization?.owner?.name ?? '';
  }

  get organizationOwnerAvatar() {
    return this.organization?.owner?.avatar ?? '';
  }

  get organizationTeam() {
    return this.organization?.team?.name ?? '';
  }

  get organizationUsers() {
    return this.organization?.users ?? {};
  }

  get organizationUsersCount() {
    return Object.keys(this.organizationUsers).length;
  }

  get parentTags() {
    return this.feature?.tags ?? [];
  }

  get commandMaxLen() {
    const SLUG_PADDING = 5;
    const SCREEN_PADDING = 18;
    const USER_PADDING = 16;
    const FLAG_PADDING = 10;
    const DATE_PADDING = 14;

    const targetsMax = this.targets.map((target) => {
      const [screen, _, flag, date] = target.split(';');
      return Math.max(screen.length + SCREEN_PADDING, flag.length + FLAG_PADDING, date.length + DATE_PADDING);
    });

    const usersMax = Object.keys(this.organizationUsers).map((user) => user.length + USER_PADDING);

    return Math.max(this.organizationSlug.length + SLUG_PADDING, ...usersMax, ...targetsMax);
  }

  get flags() {
    return Array.from(new Set(this._targets.flatMap((t) => t.flag.split(',').filter(Boolean))));
  }

  get mockDates() {
    return Array.from(new Set(this._targets.flatMap((t) => t.date.split(',').filter(Boolean))));
  }

  get allTags() {
    return this.tags.concat(this.parentTags);
  }

  get isBroken() {
    return this.allTags.some((s) => s.includes('broken'));
  }
}
