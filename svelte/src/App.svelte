<script lang="ts">
  import Modal from 'svelte-simple-modal';
  import { ENEMIES, VILLAGERS } from './combatants';
  import SetupEditor from './editor/SetupEditor.svelte';
  import Results from './results/Results.svelte';
  import SimulationSettings from './sidebar/SimulationSettings.svelte';
  import ViewSettings from './sidebar/ViewSettings.svelte';
  import type { CombatantSetup } from './types';
  import { localStorageStore } from './utils';
  import type { MsgFromWorker, MsgToWorker, StatsWithSetup } from './worker';
  import SimulationWorker from './worker?worker';

  let monthLength = localStorageStore('monthLength', 90);
  let monthStart = localStorageStore('monthStart', 0);
  let iterations = localStorageStore('iterations', 1_000);
  let findMonthStart = localStorageStore('findMonthStart', true);
  let findMonthStartStep = localStorageStore('findMonthStartStep', 15);
  let onlyShowOptimal = localStorageStore('onlyShowOptimal', true);

  let running = false;
  let progress = 0;

  let tab: 'editor' | 'results' = 'editor';

  let villagerSetup = localStorageStore<CombatantSetup[]>('villagerSetup', [
    { ...VILLAGERS[5]!, count: 8, vary: true, min_count: 8, max_count: 20 },
  ]);
  let enemySetup = localStorageStore<CombatantSetup[]>('enemySetup', [
    {
      ...ENEMIES[2]!,
      count: 1,
    },
  ]);

  let worker: Worker | null = null;
  let results: StatsWithSetup[] = [];
  let resultsWidget: Results | undefined;

  let terminateWorkerTimeout: ReturnType<typeof setTimeout> | null = null;

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
            progress += msg.progress;
            if (msg.newResult) {
              if (msg.replace) results[results.length - 1] = msg.newResult;
              else results.push(msg.newResult);
              results = results;
            }
            break;
          case 'done':
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
        iterations: $iterations,
        villagerSetup: $villagerSetup,
        enemySetup: $enemySetup,
        monthLength: $monthLength,
        monthStart: $monthStart,
      },
      findMonthStart: $findMonthStart,
      findMonthStartStep: $findMonthStartStep,
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
        <SetupEditor
          bind:villagerSetup={$villagerSetup}
          bind:enemySetup={$enemySetup}
        />
      {:else}
        <Results
          {results}
          bind:this={resultsWidget}
          onlyShowOptimal={$onlyShowOptimal}
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
        bind:iterations={$iterations}
        bind:monthLength={$monthLength}
        bind:monthStart={$monthStart}
        bind:findMonthStart={$findMonthStart}
        bind:findMonthStartStep={$findMonthStartStep}
        villagerSetup={$villagerSetup}
        enemySetup={$enemySetup}
        {running}
        {cancel}
        {run}
      />
    </aside>
    <aside class="settings-view card">
      <ViewSettings bind:onlyShowOptimal={$onlyShowOptimal} />
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
    grid-template-rows: 80px 60px auto 1fr;
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
    margin-top: 16px;
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
    top: 80px;
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
