export interface StepDefinition {
  id: string;
  keyword: 'Given' | 'When' | 'Then' | 'And' | 'But';
  pattern: string;
  file_path: string;
  line_number: number;
  category: string;
  is_problematic: boolean;
  problem_reason: string | null;
}

export type StepCategory =
  | 'Navigation'
  | 'Waits'
  | 'Assertions'
  | 'Data Setup'
  | 'Flags'
  | 'Actions'
  | 'Other';

export const STEP_CATEGORIES: StepCategory[] = [
  'Navigation',
  'Waits',
  'Assertions',
  'Data Setup',
  'Flags',
  'Actions',
  'Other',
];

export const CATEGORY_COLORS: Record<StepCategory, string> = {
  'Navigation': 'badge-primary',
  'Waits': 'badge-warning',
  'Assertions': 'badge-success',
  'Data Setup': 'badge-info',
  'Flags': 'badge-secondary',
  'Actions': 'badge-accent',
  'Other': 'badge-neutral',
};
