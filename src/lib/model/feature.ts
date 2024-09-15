import type { Store } from '$lib/store';
import Model from '$lib/model';

export default class Feature extends Model {
  id: string;
  name: string;
  description: string;
  filePath: string;
  tags: string[];

  constructor(store: Store, data: RustyFeature) {
    super(store);
    this.id = data.id;
    this.name = data.name;
    this.description = data.description;
    this.filePath = data.file_path;
    this.tags = data.tags;
  }

  get scenarios() {
    return this.store.scenarios.filter((s) => s.feature === this);
  }

  get organization() {
    return this.scenarios[0]?.organization;
  }

  get organizationId() {
    return this.organization?.id ?? '';
  }

  get organizationName() {
    return this.organization?.name ?? '';
  }

  get targets() {
    return Array.from(new Set(...this.scenarios.map((s) => s.targets)));
  }

  get organizationUsers() {
    return this.organization?.users ?? {};
  }

  get organizationUsersCount() {
    return Object.keys(this.organizationUsers).length;
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

  get commandMaxLen() {
    return Math.max(...this.scenarios.map((s) => s.commandMaxLen));
  }

  get flags() {
    return Array.from(new Set(...this.scenarios.map((s) => s.flags)));
  }

  get mockDates() {
    return Array.from(new Set(...this.scenarios.map((s) => s.mockDates)));
  }

  get isBroken() {
    return this.tags.some((s) => s.includes('broken'));
  }
}
