interface RustyPerson {
  name: string;
  avatar: string;
  teams: string[];
}

interface RustyTeam {
  name: string;
}

interface RustyOrganization {
  id: string;
  name: string;
  blame: string;
  team: string;
  users: Record<string, string>;
}

interface RustyScenario {
  id: string;
  name: string;
  description: string;
  targets: string[];
  steps: number;
  examples: number;
  tags: string[]
  feature_id: string;
  organization_name: string;
}

interface RustyFeature {
  id: string,
  name: string,
  description: string,
  file_path: string,
  tags: string[],
}

interface RustyFindE2eRepo {
  api_url: string;
  app_url: string;
  location: string;
  password: string;
}

type InvokeErr = { err: string };
type InvokeFindE2eRepo = { ok: RustyFindE2eRepo };
type InvokeGetOrganizations = { people: RustyPerson[]; teams: RustyTeam[]; organizations: RustyOrganization[]; };
type InvokeGetFeatures = { features: RustyFeature[]; scenarios: RustyScenario[]; }
