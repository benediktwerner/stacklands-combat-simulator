import type { CombatantSetup } from './types';

export const sleep = (): Promise<void> => {
  return new Promise((resolve) => setTimeout(resolve));
};

export const calculateVariations = (cs: CombatantSetup[]): number => {
  let result = 1;
  for (const c of cs) {
    if (c.vary) {
      result *= Math.max(1, c.max_count - c.min_count + 1);
    }
  }
  return result;
};
