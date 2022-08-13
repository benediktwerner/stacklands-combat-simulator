<script lang="ts">
  import CombatantImage from './CombatantImage.svelte';
  import { VILLAGERS, ENEMIES } from '../combatants';
  import StatsTable from './StatsTable.svelte';
  import type { Combatant } from '../types';

  export let add: (c: Combatant) => void;
  export let isEnemy: boolean;

  const defaultCustom: Combatant = {
    name: '',
    hp: 5,
    attack_speed: 2.5,
    hit_chance: 50,
    min_damage: 1,
    max_damage: 3,
  };
  let custom: Combatant | null = null;
</script>

<h2>Add Combatant</h2>

{#if custom}
  <table>
    <tr>
      <td>Name:</td>
      <td><input type="text" bind:value={custom.name} /></td>
    </tr>
    <tr>
      <td>HP:</td>
      <td
        ><input
          type="number"
          min="0"
          step="1"
          max="9999"
          bind:value={custom.hp}
        /></td
      >
    </tr>
    <tr>
      <td>Attack Speed:</td>
      <td
        ><input
          type="number"
          min="0.1"
          step="0.1"
          max="99"
          bind:value={custom.attack_speed}
        /></td
      >
    </tr>
    <tr>
      <td>Hit Chance:</td>
      <td
        ><input
          type="number"
          min="0"
          step="1"
          max="99"
          bind:value={custom.hit_chance}
        /> %</td
      >
    </tr>
    <tr>
      <td>Damage:</td>
      <td>
        <input
          type="number"
          min="0"
          max="99"
          step="1"
          bind:value={custom.min_damage}
        />
        &ndash;
        <input
          type="number"
          min="0"
          max="99"
          step="1"
          bind:value={custom.max_damage}
        />
      </td>
    </tr>
  </table>
  <div
    class="button-container"
    on:click={() => {
      custom.attack_speed *= 10;
      custom.hit_chance /= 100;
      if (!custom.name) custom.name = 'Custom';
      add(custom);
    }}
  >
    <button class="button button-primary">Add</button>
  </div>
{:else}
  <div class="container">
    {#each isEnemy ? ENEMIES : VILLAGERS as combatant}
      <div class="combatant-row" on:click={() => add(combatant)}>
        <div class="combatant-side">
          <CombatantImage {combatant} {isEnemy} />
          <b>{combatant.name}</b>
        </div>
        <StatsTable {combatant} />
      </div>
    {/each}
  </div>
  <hr />
  <div class="combatant-row" on:click={() => (custom = defaultCustom)}>
    <b>Add Custom Combatant</b>
  </div>
{/if}

<style>
  h2 {
    margin-top: 0;
  }

  .container {
    max-height: 70vh;
    overflow-y: scroll;
  }

  .button-container {
    display: flex;
    justify-content: center;
    margin-top: 24px;
  }

  .combatant-row {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 24px;
    cursor: pointer;
    border-radius: 12px;
    padding: 12px;
  }
  .combatant-row:hover {
    background-color: var(--card-bg-hover);
  }
  .combatant-side {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 7px;
  }

  td:nth-child(2) {
    text-align: right;
  }
</style>
