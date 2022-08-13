import type { CombatantStats } from './wasm/stacklands_combat_simulator';

export interface Combatant extends CombatantStats {
  name: string;
}
export interface CombatantSetup extends Combatant {
  count: number;
}
