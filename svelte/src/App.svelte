<script lang="ts">
  import Modal from 'svelte-simple-modal';
  import Results from './results/Results.svelte';
  import Sidebar from './sidebar/Sidebar.svelte';
  import SimulationWorker from './worker?worker';
  import type { MsgFromWorker, MsgToWorker, StatsWithSetup } from './worker';
  import SetupEditor from './editor/SetupEditor.svelte';
  import type { CombatantSetup } from './types';
  import { ENEMIES, VILLAGERS } from './combatants';
  import type { CombatantStats } from './wasm/stacklands_combat_simulator';

  let monthLength = 120;
  let monthStart = 0;
  let iterations = 1_000;
  let findMonthStart = true;
  let running = false;
  let progress = 0;

  let tab: 'editor' | 'results' = 'editor';

  let villagerSetup: CombatantSetup[] = [{ ...VILLAGERS[5], count: 8 }];
  let enemySetup: CombatantSetup[] = [
    {
      ...ENEMIES[2],
      count: 1,
    },
  ];

  let worker: Worker = null;
  let results: StatsWithSetup[] = [];
  let resultsWidget: Results | undefined;

  let terminateWorkerTimeout: ReturnType<typeof setTimeout> | null = null;

  const addResult = (result?: StatsWithSetup) => {
    if (result) {
      results.push(result);
      results = results;
    }
  };

  const createSetup = (combatants: CombatantSetup[]): CombatantStats[] => {
    const result = [];
    for (const c of combatants) {
      for (let i = 0; i < c.count; i++) result.push(c);
    }
    return result;
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
            }
            break;
        }
      };
    }
    const msg: MsgToWorker = {
      type: 'simulate',
      setup: {
        iterations,
        villagerSetup: createSetup(villagerSetup),
        enemySetup: createSetup(enemySetup),
        monthLength,
        monthStart,
      },
      findMonthStart,
    };
    worker.postMessage(msg);
  };

  const cancel = () => {
    running = false;
    if (worker !== null) {
      worker.postMessage({ type: 'cancel' } as MsgToWorker);
      terminateWorkerTimeout = setTimeout(() => {
        worker.terminate();
        worker = null;
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
        <Results {results} bind:this={resultsWidget} />
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
</Modal>

<style>
  #container {
    display: grid;
    grid-template-areas:
      'heading heading'
      'main nav'
      'main side';
    grid-template-rows: 100px 60px auto;
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
