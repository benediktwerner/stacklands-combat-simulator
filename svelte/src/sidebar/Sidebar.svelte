<script lang="ts">
  import type { CombatantSetup } from '../types';
  import { calculateVariations } from '../utils';

  import SliderWithValue from './SliderWithValue.svelte';

  export let monthLength: number;
  export let monthStart: number;
  export let iterations: number;
  export let findMonthStart: boolean;
  export let findMonthStartStep: number;
  export let running: boolean;
  export let villagerSetup: CombatantSetup[];
  export let enemySetup: CombatantSetup[];
  export let cancel: () => any;
  export let run: () => any;

  $: combatantVariations =
    calculateVariations(villagerSetup) * calculateVariations(enemySetup);

  $: {
    const maxStep = Math.floor(monthLength / 2);
    if (findMonthStartStep > maxStep) findMonthStartStep = maxStep;
  }
</script>

<h3>Settings</h3>

<table>
  <tr>
    <td> Month Length: </td>
    <td>
      <select bind:value={monthLength}>
        <option value={90}>Short (90s)</option>
        <option value={120}>Normal (120s)</option>
        <option value={200}>Long (200s)</option>
      </select>
    </td>
  </tr>
  <tr title="Try various month start times to find the best one">
    <td>Find optimal start:</td>
    <td><input type="checkbox" bind:checked={findMonthStart} /></td>
  </tr>
  {#if findMonthStart}
    <tr
      title="How many seconds between each month start time attempt. A lower amount means more accuracy but it takes longer since it leads to more options to check."
    >
      <td>Start steps:</td>
      <td class="center">
        <SliderWithValue
          bind:value={findMonthStartStep}
          min={5}
          max={Math.floor(monthLength / 2)}
          step={5}
        />
      </td>
    </tr>
  {:else}
    <tr>
      <td> Month Start: </td>
      <td class="center">
        <SliderWithValue
          bind:value={monthStart}
          max={monthLength}
          step={5}
          showMax
        />
      </td>
    </tr>
  {/if}
  <tr>
    <td> Iterations: </td>
    <td>
      <input type="number" min="1" max="100000000" bind:value={iterations} />
    </td>
  </tr>
  <tr
    title="How many different possible scenarios will be tested with the current setup (the higher the longer the simulation takes)."
  >
    <td>Max scenarios:</td>
    <td>
      {combatantVariations *
        (findMonthStart ? Math.floor(monthLength / findMonthStartStep) : 1)}
    </td>
  </tr>
</table>

<div class="button-container">
  <button class="button button-primary" on:click={running ? cancel : run}>
    {running ? 'Cancel' : 'Simulate'}
  </button>
</div>

<style>
  h3 {
    margin-top: 0;
  }

  table,
  tr {
    width: 100%;
  }
  td:nth-child(2) {
    text-align: right;
  }

  .button-container {
    margin-top: 24px;
    display: flex;
    justify-content: center;
  }
</style>
