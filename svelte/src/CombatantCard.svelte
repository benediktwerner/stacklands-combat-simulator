<script lang="ts">
  import CombatantImage from './CombatantImage.svelte';
  import StatsTable from './StatsTable.svelte';
  import type { CombatantSetup } from './types';

  export let combatant: CombatantSetup;
  export let isEnemy: boolean = false;
  export let remove: (c: CombatantSetup) => void;
</script>

<div class="container">
  <CombatantImage {combatant} {isEnemy} />
  <b>{combatant.name}</b>
  <StatsTable {combatant} />
  <div class="row">
    <button
      class:red={combatant.count === 1}
      on:click={(e) => {
        if (combatant.count === 1) remove(combatant);
        else {
          combatant.count -= e.shiftKey ? 10 : 1;
          if (combatant.count < 1) combatant.count = 1;
        }
      }}
      title="Hold Shift to decrease by 10">&minus;</button
    >
    {combatant.count}
    <button
      on:click={(e) => {
        combatant.count += e.shiftKey ? 10 : 1;
      }}
      title="Hold Shift to increase by 10">+</button
    >
  </div>
</div>

<style>
  .container {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 200px;
    background-color: var(--card-bg-hover);
    border-radius: 12px;
    padding: 24px;
    padding-bottom: 20px;
    gap: 7px;
  }

  .row {
    display: flex;
    gap: 12px;
  }

  button {
    font-weight: bold;
    border-radius: 12px;
    padding: 0 10px;
  }
  .red {
    background-color: var(--bad);
  }
</style>
