import type { Store } from '$lib/store';
import Model from '$lib/model';

export default class Team extends Model {
  name: string;

  constructor(store: Store, data: RustyTeam) {
    super(store);
    this.name = data.name;
  }

  get members() {
    return this.store.owners.filter((o) => o.isInTeam(this.name));
  }

  get organizations() {
    return this.store.organizations.filter((o) => o.teamName === this.name);
  }

  get scenarios() {
    return this.store.scenarios.filter((s) => s.organizationTeam === this.name);
  }

  get features() {
    return this.store.features.filter((f) =>
      f.scenarios.some((s) => s.organizationTeam === this.name)
    );
  }
}
