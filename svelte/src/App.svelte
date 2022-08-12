<script lang="ts">
  import Results from './Results.svelte';
  import Sidebar from './Sidebar.svelte';
  import init, {
    simulate,
    type Stats,
  } from './wasm/stacklands_combat_simulator.js';

  let monthLength = 120;
  let monthStart = 0;
  let iterations = 10_000;

  let results: Stats | null = null;
  results = {
    iters: 10000,
    wins: 893,
    total_length: 852318,
    longest: 115,
    village_survivors: [81, 134, 186, 205, 158, 86, 36, 7],
    enemy_hp: [1870, 2962, 2587, 1348, 314, 25, 1, 0, 0, 0],
  };
  results.enemy_hp.reverse();

  const initPromise = init();

  const run = async () => {
    await initPromise;
    const swordsmen = {
      hp: 7,
      attack_speed: 15,
      hit_chance: 0.9,
      min_damage: 2,
      max_damage: 2,
    };
    const demon_lord = {
      hp: 666,
      attack_speed: 15,
      hit_chance: 0.75,
      min_damage: 1,
      max_damage: 3,
    };
    const villagers = Array(8).fill(swordsmen);
    const enemies = [demon_lord];
    console.log(monthStart, monthLength);
    const res = simulate(
      iterations,
      villagers,
      enemies,
      monthStart,
      monthLength
    );
    res.enemy_hp.reverse();
    results = res;
  };
</script>

<div id="container">
  <h1>Stacklands Combat Simulator</h1>
  <main class="card">
    {#if results !== null}
      <Results {results} />
    {/if}
  </main>
  <aside class="card">
    <Sidebar bind:iterations bind:monthLength bind:monthStart {run} />
  </aside>

  <a
    id="source-link"
    href="https://github.com/benediktwerner/stacklands-combat-simulator"
    target="_blank"
  >
    Source Code
  </a>
</div>

<style>
  #container {
    display: grid;
    grid-template-areas:
      'heading heading'
      'main side';
    grid-template-rows: 100px auto;
    grid-template-columns: auto 350px;
    width: 100vw;
    height: 100vh;
    margin: 0;
    gap: 24px;
    padding: 24px;
    padding-top: 0;
  }

  h1 {
    grid-area: heading;
    text-align: center;
    font-size: 3.2em;
    line-height: 1.1;
  }

  main {
    grid-area: main;
  }

  aside {
    grid-area: side;
  }

  #source-link {
    position: absolute;
    top: 0;
    right: 0;
    padding: 8px;

    display: flex;
    align-items: center;
    justify-content: center;
    gap: 10px;

    border-bottom-left-radius: 12px;
    background: var(--card-bg);
    color: var(--text-muted);
    text-decoration: none;
  }
  #source-link:hover {
    color: var(--text-muted-hover);
  }
</style>
