import type { Store } from '$lib/store';
import Model from '$lib/model';

export default class Owner extends Model {
  name: string;
  _avatar: string;
  _teams: Set<string>;

  constructor(store: Store, data: RustyPerson) {
    super(store);
    this.name = data.name;
    this._avatar = data.avatar;
    this._teams = new Set(data.teams);
  }

  isInTeam(team: string) {
    return this._teams.has(team);
  }

  get teams() {
    return Array.from(this._teams)
      .sort()
      .map((t) => this.store.findTeam(t))
      .filter((t) => !!t);
  }

  get avatar() {
    return this._avatar || '/unknown-avatar.png';
  }

  get firstTeam() {
    return this.teams[0];
  }

  get organizations() {
    return this.store.organizations.filter((o) => o.ownerName === this.name);
  }

  get scenarios() {
    return this.store.scenarios.filter(
      (s) => s.organizationOwner === this.name
    );
  }

  get features() {
    return this.store.features.filter((f) =>
      f.scenarios.some((s) => s.organizationOwner === this.name)
    );
  }
}
