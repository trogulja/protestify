import type {Store} from '$lib/store';

export default class Model {
  store: Store;

  constructor(store: Store) {
    this.store = store;
  }
}
