import type { Store } from '$lib/store';
import Model from '$lib/model';

export default class Organization extends Model {
  id: string;
  name: string;
  ownerName: string;
  teamName: string;
  users: Record<string, string>;

  constructor(store: Store, data: RustyOrganization) {
    super(store);
    this.id = data.id;
    this.name = data.name;
    this.ownerName = data.blame;
    this.teamName = data.team;
    this.users = data.users;
  }

  get owner() {
    return this.store.owners.find((o) => o.name === this.ownerName);
  }

  get team() {
    return this.store.teams.find((t) => t.name === this.teamName);
  }

  get scenarios() {
    return this.store.scenarios.filter((s) => s.organization === this);
  }
}
