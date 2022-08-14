<script lang="ts">
  import Modal from 'svelte-simple-modal';
  import { ENEMIES, VILLAGERS } from './combatants';
  import SetupEditor from './editor/SetupEditor.svelte';
  import Results from './results/Results.svelte';
  import SimulationSettings from './sidebar/SimulationSettings.svelte';
  import ViewSettings from './sidebar/ViewSettings.svelte';
  import type { CombatantSetup } from './types';
  import type { MsgFromWorker, MsgToWorker, StatsWithSetup } from './worker';
  import SimulationWorker from './worker?worker';

  let monthLength = 90;
  let monthStart = 0;
  let iterations = 1_000;
  let findMonthStart = true;
  let findMonthStartStep = 15;
  let running = false;
  let progress = 0;
  let onlyShowOptimal = true;

  let tab: 'editor' | 'results' = 'editor';

  let villagerSetup: CombatantSetup[] = [
    { ...VILLAGERS[5], count: 8, vary: true, min_count: 8, max_count: 20 },
  ];
  let enemySetup: CombatantSetup[] = [
    {
      ...ENEMIES[2],
      count: 1,
    },
  ];

  let worker: Worker | null = null;
  let results: StatsWithSetup[] = [];
  let resultsWidget: Results | undefined;

  let terminateWorkerTimeout: ReturnType<typeof setTimeout> | null = null;

  const addResult = (result?: StatsWithSetup) => {
    if (result) {
      results.push(result);
      results = results;
    }
  };

  const run = () => {
    tab = 'results';

    running = true;
    progress = 0;
    results = [];
    resultsWidget?.reset();

    if (worker === null) {
      worker = new SimulationWorker();
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
          case 'cancelled':
            if (terminateWorkerTimeout) {
              clearTimeout(terminateWorkerTimeout);
              terminateWorkerTimeout = null;
              progress = 100;
            }
            break;
        }
      };
    }
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
      findMonthStartStep,
    };
    worker.postMessage(msg);
  };

  const cancel = () => {
    running = false;
    if (worker !== null) {
      worker.postMessage({ type: 'cancel' } as MsgToWorker);
      terminateWorkerTimeout = setTimeout(() => {
        worker?.terminate();
        worker = null;
        progress = 100;
      }, 1000);
    }
  };
</script>

<Modal>
  <div id="container">
    <h1>Stacklands Combat Simulator</h1>
    <main class="card">
      {#if tab === 'editor'}
        <SetupEditor bind:villagerSetup bind:enemySetup />
      {:else}
        <Results
          {results}
          bind:this={resultsWidget}
          onlyShowOptimal={onlyShowOptimal && findMonthStart}
        />
      {/if}
    </main>
    <nav class="card">
      <button class="button" on:click={() => (tab = 'editor')}>
        Edit Setup
      </button>
      <button class="button" on:click={() => (tab = 'results')}>
        View Results
      </button>
    </nav>
    <aside class="settings-simulation card">
      <SimulationSettings
        bind:iterations
        bind:monthLength
        bind:monthStart
        bind:findMonthStart
        bind:findMonthStartStep
        {villagerSetup}
        {enemySetup}
        {running}
        {cancel}
        {run}
      />
    </aside>
    <aside class="settings-view card">
      <ViewSettings bind:onlyShowOptimal />
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
</Modal>

<style>
  #container {
    display: grid;
    grid-template-areas:
      'heading heading'
      'main nav'
      'main side1'
      'main side2';
    grid-template-rows: 100px 60px auto 1fr;
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

  nav {
    grid-area: nav;
    display: flex;
    justify-content: space-evenly;
    padding: 12px !important;
  }

  nav button {
    background-color: var(--card-bg-hover);
  }
  nav button:hover {
    background-color: var(--card-bg-hover-hover);
  }

  .settings-simulation {
    grid-area: side1;
  }
  .settings-view {
    grid-area: side2;
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
