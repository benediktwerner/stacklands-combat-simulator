<script lang="ts">
  import { getContext } from 'svelte';
  import AddCombatantDialog from './AddCombatantDialog.svelte';

  import CombatantCard from './CombatantCard.svelte';
  import type { SimpleModalContext } from '../simple-modal';
  import type { CombatantSetup } from '../types';
  import type { CombatantStats } from '../wasm/stacklands_combat_simulator';

  let villagerSetup: CombatantSetup[] = [
    {
      hp: 7,
      attack_speed: 15,
      hit_chance: 0.9,
      min_damage: 2,
      max_damage: 2,
      name: 'Swordman',
      count: 8,
    },
    {
      hp: 5,
      attack_speed: 3,
      hit_chance: 0.4,
      min_damage: 1,
      max_damage: 2,
      name: 'Villager',
      count: 1,
    },
  ];
  let enemySetup: CombatantSetup[] = [
    {
      hp: 666,
      attack_speed: 15,
      hit_chance: 0.75,
      min_damage: 1,
      max_damage: 3,
      name: 'Demon Lord',
      count: 1,
    },
  ];

  const modal = getContext<SimpleModalContext>('simple-modal');
  const addCard = (isEnemy: boolean) => {
    modal.open(
      AddCombatantDialog,
      {
        add: (c) => {
          const n = { ...c, count: 1 };
          if (isEnemy) {
            enemySetup.push(n);
            enemySetup = enemySetup;
          } else {
            villagerSetup.push(n);
            villagerSetup = villagerSetup;
          }
          modal.close();
        },
        isEnemy,
      },
      {
        closeButton: false,
        styleContent: { padding: '24px' },
        styleWindow: {
          background: 'var(--card-bg)',
          color: 'var(--text)',
          width: '350px',
        },
      }
    );
  };

  const createSetup = (combatants: CombatantSetup[]): CombatantStats[] => {
    const result = [];
    for (const c of combatants) {
      for (let i = 0; i < c.count; i++) result.push(c);
    }
    return result;
  };
  export const getVillagers = (): CombatantStats[] =>
    createSetup(villagerSetup);
  export const getEnemies = (): CombatantStats[] => createSetup(enemySetup);
</script>

<div class="row">
  {#each villagerSetup as villager}
    <CombatantCard
      combatant={villager}
      remove={(c) => (villagerSetup = villagerSetup.filter((x) => x !== c))}
    />
  {/each}

  <button class="new" on:click={() => addCard(false)}>
    <img
      src="/images/plus.png"
      alt="Add new combatant"
      title="Add combatant"
      width="100"
      height="100"
    />
  </button>
</div>

<div class="divider" />

<div class="row">
  {#each enemySetup as enemy}
    <CombatantCard
      combatant={enemy}
      isEnemy={true}
      remove={(c) => (enemySetup = enemySetup.filter((x) => x !== c))}
    />
  {/each}

  <button class="new" on:click={() => addCard(true)}>
    <img
      src="/images/plus.png"
      alt="Add new combatant"
      title="Add combatant"
      width="100"
      height="100"
    />
  </button>
</div>

<style>
  .row {
    display: flex;
    flex-wrap: wrap;
    gap: 12px;
  }

  .divider {
    width: 90%;
    border: 0;
    height: 1px;
    overflow: visible;
    border-top: 2px solid currentColor;
    text-align: center;
    margin: 25px 5%;
  }
  .divider::after {
    content: 'VS';
    display: inline-block;
    position: relative;
    top: -0.7em;
    font-size: 1em;
    background-color: var(--card-bg);
    padding: 0px 12px;
  }

  .new {
    background-color: transparent;
    width: 200px;
  }
  .new img {
    font-size: 100px;
    background-color: var(--card-bg);
    border-radius: 50%;
    transition: background-color 150ms;
  }
  .new:hover img {
    background-color: var(--card-bg-hover);
  }
</style>
