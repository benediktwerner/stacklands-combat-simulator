<script lang="ts">
  import type { CombatantSetup } from '../types';
  import CombatantImage from './CombatantImage.svelte';
  import StatsTable from './StatsTable.svelte';

  export let combatant: CombatantSetup;
  export let isEnemy: boolean = false;
  export let remove: (c: CombatantSetup) => void;
</script>

<div class="container">
  <button class="delete" on:click={() => remove(combatant)}>❌</button>
  <CombatantImage {combatant} {isEnemy} />
  <b>{combatant.name}</b>
  <StatsTable {combatant} />
  <div class="row" class:vary={!!combatant.vary}>
    {#if combatant.vary}
      <input
        type="number"
        bind:value={combatant.min_count}
        min="1"
        max={combatant.max_count}
        step="1"
      />
      &ndash;
      <input
        type="number"
        bind:value={combatant.max_count}
        min={combatant.min_count}
        maxlength="2"
        max="99"
        step="1"
      />
    {:else}
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
    {/if}
    <button
      class="switch"
      title={combatant.vary
        ? 'Switch to setting a single value'
        : 'Switch to setting a range of values'}
      on:click={() => {
        combatant.vary = !combatant.vary;
        if (combatant.vary) {
          combatant.min_count = combatant.count;
          combatant.max_count = combatant.count;
        }
      }}
    >
      {combatant.vary ? '1' : '↔'}
    </button>
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
    position: relative;
  }

  .row {
    display: flex;
    gap: 12px;
  }
  .row.vary {
    gap: 3px;
  }

  button {
    font-weight: bold;
    border-radius: 12px;
    padding: 0 10px;
  }
  .red {
    background-color: var(--bad);
  }
  .delete {
    position: absolute;
    background-color: transparent;
    padding: 0;
    top: 6px;
    right: 6px;
    font-size: 1em;
  }
  .switch {
    position: absolute;
    right: 12px;
    height: 24px;
  }

  input[type='number'] {
    width: 40px;
  }
</style>
