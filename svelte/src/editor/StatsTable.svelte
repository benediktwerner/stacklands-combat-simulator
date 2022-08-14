<script lang="ts">
  import type { CombatantStats } from '../wasm/stacklands_combat_simulator';

  export let combatant: CombatantStats;
</script>

<table>
  <tr>
    <td>HP:</td>
    <td>{combatant.hp}</td>
  </tr>
  <tr>
    <td>Attack Speed:</td>
    <td>{combatant.attack_speed / 10}s</td>
  </tr>
  <tr>
    <td>Hit Chance:</td>
    <td> {combatant.hit_chance * 100}%</td>
  </tr>
  <tr>
    <td>Damage:</td>
    <td>
      {combatant.min_damage}{combatant.min_damage !== combatant.max_damage
        ? '-' + combatant.max_damage
        : ''}
    </td>
  </tr>
  <tr>
    <td>Stun Chance:</td>
    <td> {combatant.stun_chance * 100}%</td>
  </tr>
</table>

<style>
  td:nth-child(2) {
    text-align: right;
    padding-left: 7px;
  }
</style>
