<script lang="ts">
  import Results from './Results.svelte';
  import Sidebar from './Sidebar.svelte';
  import SimulationWorker from './worker?worker';
  import type { MsgFromWorker, MsgToWorker, StatsWithSetup } from './worker';

  let monthLength = 120;
  let monthStart = 0;
  let iterations = 10_000;
  let findMonthStart = true;
  let running = false;
  let progress = 0;

  let worker: Worker = null;
  let results: StatsWithSetup[] | undefined;
  let resultsWidget: Results | undefined;

  const addResult = (result?: StatsWithSetup) => {
    if (result) {
      results.push(result);
      results = results;
    }
  };

  const run = () => {
    running = true;
    progress = 0;
    results = [];
    resultsWidget?.reset();

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
    const villagerSetup = Array(8).fill(swordsmen);
    const enemySetup = [demon_lord];

    if (worker === null) worker = new SimulationWorker();
    worker.onmessage = (e: MessageEvent<MsgFromWorker>) => {
      const msg = e.data;
      switch (msg.type) {
        case 'progress':
          progress = msg.progress;
          addResult(msg.newResult);
          break;
        case 'done':
          addResult(msg.newResult);
          progress = 100;
          running = false;
          break;
      }
    };
    const msg: MsgToWorker = {
      type: 'simulate',
      setup: {
        iterations,
        villagerSetup,
        enemySetup,
        monthLength,
        monthStart,
      },
      findMonthStart,
    };
    worker.postMessage(msg);
  };

  const cancel = () => {
    running = false;
    if (worker !== null) worker.postMessage({ type: 'cancel' } as MsgToWorker);
  };
</script>

<div id="container">
  <h1>Stacklands Combat Simulator</h1>
  <main class="card">
    {#if results}
      <Results {results} bind:this={resultsWidget} />
    {/if}
  </main>
  <aside class="card">
    <Sidebar
      bind:iterations
      bind:monthLength
      bind:monthStart
      bind:findMonthStart
      {running}
      {cancel}
      {run}
    />
  </aside>

  <a
    id="source-link"
    href="https://github.com/benediktwerner/stacklands-combat-simulator"
    target="_blank"
  >
    Source Code
  </a>
</div>

<div class="progress-bar">
  <div class="progress-bar-inner" style:width="{progress}%" />
</div>

<style>
  #container {
    display: grid;
    grid-template-areas:
      'heading heading'
      'main side';
    grid-template-rows: 100px auto;
    grid-template-columns: auto 350px;
    width: 100%;
    min-height: 100vh;
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

  .progress-bar {
    position: absolute;
    left: 24px;
    right: 24px;
    top: 100px;
    height: 12px;
    border-radius: 12px;
    background-color: var(--card-bg);
  }
  .progress-bar-inner {
    height: 100%;
    border-radius: 12px;
    background: var(--highlight);
    transition: width 250ms;
  }
</style>
