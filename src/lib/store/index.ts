import { get } from 'svelte/store';
import { invoke } from "@tauri-apps/api/core";
import { addToast, ToastType } from '$lib/store/toasts';
import settings from '$lib/store/settings';
import { isInvokeErr } from '$lib/utils';

import Owner from "$lib/model/owner";
import Team from "$lib/model/team";
import Organization from "$lib/model/organization";
import Feature from "$lib/model/feature";
import Scenario from "$lib/model/scenario";

export class Store {
  isLoaded: boolean = false;

  scenarios: Scenario[] = [];
  features: Feature[] = [];
  organizations: Organization[] = [];
  owners: Owner[] = [];
  teams: Team[] = [];

  get tableData() {
    const store = this;
    return {
      get scenarios() {
        return store.scenarios.map((s) => ({
          scenario: s.name,
          scenarioId: s.id,
          feature: s.featureName,
          featureId: s.featureId,
          organization: s.organizationName,
          organizationId: s.organizationId,
          steps: s.steps,
          examples: s.examples,
          file: s.featureFilePath,
          tags: s.allTags,
          owner: s.organizationOwner,
          ownerAvatar: s.organizationOwnerAvatar,
          team: s.organizationTeam,
          isBroken: s.isBroken,
        }));
      }
    }
  }

  async loadData() {
    this.isLoaded = false;

    await settings.init();
    const {basePath} = get(settings);

    if (!basePath) return this.handleError('base path not found');

    const organizationPath = { filePath: `${basePath}/data` };
    const featurePath = { basePath: `${basePath}/features` };

    const isBasePathValid = await invoke<boolean>('validate_e2e_repo', { path: basePath });
    if (!isBasePathValid) return this.handleError('e2e repo not found');

    const orgs = await invoke<InvokeGetOrganizations | InvokeErr>('get_organizations', organizationPath);
    if (isInvokeErr(orgs)) return this.handleError(orgs.err);
    this.parseOrgs(orgs);

    const data = await invoke<InvokeGetFeatures | InvokeErr>('get_features', featurePath);
    if (isInvokeErr(data)) return this.handleError(data.err);
    this.parseFeatures(data.features);
    this.parseScenarios(data.scenarios);

    this.isLoaded = true;
  }

  async reloadData() {
    addToast({
      type: ToastType.INFO,
      message: 'Reloading data...',
    });
    this.isLoaded = false;
    await this.loadData();
  }

  findScenario(id: string) {
    return this.scenarios.find((s) => s.id === id);
  }

  findFeature(id: string) {
    return this.features.find((f) => f.id === id);
  }

  findOwner(name: string) {
    return this.owners.find((o) => o.name === name);
  }

  findTeam(name: string) {
    return this.teams.find((t) => t.name === name);
  }

  findOrganization(id: string) {
    return this.organizations.find((o) => o.id === id);
  }

  private handleError(err: string) {
    addToast({
      type: ToastType.ERROR,
      message: err,
    })
  }

  private parseOrgs(data: InvokeGetOrganizations) {
    this.owners = data.people.map((p) => new Owner(this, p));
    this.teams = data.teams.map((t) => new Team(this, t));
    this.organizations = data.organizations.map((o) => new Organization(this, o));
  }

  private parseFeatures(data: RustyFeature[]) {
    this.features = data.map((f) => new Feature(this, f));
  }

  private parseScenarios(data: RustyScenario[]) {
    this.scenarios = data.map((s) => new Scenario(this, s));
  }
}

export default new Store();
