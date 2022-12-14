import type { CombatantStats } from './wasm/stacklands_combat_simulator';

export interface Combatant extends CombatantStats {
  name: string;
  image_name?: string;
}
export interface CombatantSetup extends Combatant {
  count: number;
  min_count?: number;
  max_count?: number;
  vary?: boolean;
}
