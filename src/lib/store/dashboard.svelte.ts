import store from '$lib/store';
import type Scenario from '$lib/model/scenario';
import type Organization from '$lib/model/organization';

export interface DashboardStats {
  featureCount: number;
  scenarioCount: number;
  organizationCount: number;
  teamCount: number;
  ownerCount: number;
  brokenCount: number;
}

export interface TeamBreakdown {
  team: string;
  scenarioCount: number;
  brokenCount: number;
}

export interface OwnerBreakdown {
  owner: string;
  avatar: string;
  scenarioCount: number;
  brokenCount: number;
}

export function getStats(): DashboardStats {
  return {
    featureCount: store.features.length,
    scenarioCount: store.scenarios.length,
    organizationCount: store.organizations.length,
    teamCount: store.teams.length,
    ownerCount: store.owners.length,
    brokenCount: store.scenarios.filter(s => s.isBroken).length,
  };
}

export function getBrokenScenarios(): Scenario[] {
  return store.scenarios.filter(s => s.isBroken);
}

export function getOrgsWithoutTests(): Organization[] {
  const orgsWithTests = new Set(store.scenarios.map(s => s.organizationName));
  return store.organizations.filter(o => !orgsWithTests.has(o.name));
}

export function getTeamBreakdown(): TeamBreakdown[] {
  const breakdownMap = new Map<string, TeamBreakdown>();

  for (const scenario of store.scenarios) {
    const teamName = scenario.organizationTeam || 'Unassigned';
    const existing = breakdownMap.get(teamName);

    if (existing) {
      existing.scenarioCount++;
      if (scenario.isBroken) existing.brokenCount++;
    } else {
      breakdownMap.set(teamName, {
        team: teamName,
        scenarioCount: 1,
        brokenCount: scenario.isBroken ? 1 : 0,
      });
    }
  }

  return Array.from(breakdownMap.values()).sort((a, b) => b.scenarioCount - a.scenarioCount);
}

export function getOwnerBreakdown(): OwnerBreakdown[] {
  const breakdownMap = new Map<string, OwnerBreakdown>();

  for (const scenario of store.scenarios) {
    const ownerName = scenario.organizationOwner || 'Unassigned';
    const avatar = scenario.organizationOwnerAvatar || '';
    const existing = breakdownMap.get(ownerName);

    if (existing) {
      existing.scenarioCount++;
      if (scenario.isBroken) existing.brokenCount++;
    } else {
      breakdownMap.set(ownerName, {
        owner: ownerName,
        avatar,
        scenarioCount: 1,
        brokenCount: scenario.isBroken ? 1 : 0,
      });
    }
  }

  return Array.from(breakdownMap.values()).sort((a, b) => b.scenarioCount - a.scenarioCount);
}
