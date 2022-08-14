<script lang="ts">
  import type { StatsWithSetup } from '../worker';
  import ResultGraph from './ResultGraph.svelte';
  import ResultRow from './ResultRow.svelte';

  export let results: StatsWithSetup[];
  export let onlyShowOptimal: boolean;

  let resultIndex: number | null = null;

  $: resultsFiltered = filter(results, onlyShowOptimal);
  $: showSetup =
    results.length > 0 &&
    (results[0].villagerSetup.some((c) => c.vary) ||
      results[0].enemySetup.some((c) => c.vary));

  export const reset = () => {
    resultIndex = null;
  };

  const sameSetup = (a: StatsWithSetup, b: StatsWithSetup): boolean => {
    for (const i in a.villagerSetup) {
      if (a.villagerSetup[i].count !== b.villagerSetup[i].count) return false;
    }
    for (const i in a.enemySetup) {
      if (a.enemySetup[i].count !== b.enemySetup[i].count) return false;
    }
    return true;
  };

  const better = (n: StatsWithSetup, old: StatsWithSetup): boolean => {
    if (n.wins !== old.wins) return n.wins > old.wins;
    const nSurv = n.village_survivors
      .map((v, i) => v * (i + 1))
      .reduce((a, b) => a + b, 0);
    const oSurv = old.village_survivors
      .map((v, i) => v * (i + 1))
      .reduce((a, b) => a + b, 0);
    if (nSurv !== oSurv) return nSurv > oSurv;
    const nEnemy = n.enemy_hp
      .map((v, i) => v * (i + 1))
      .reduce((a, b) => a + b, 0);
    const oEnemy = old.enemy_hp
      .map((v, i) => v * (i + 1))
      .reduce((a, b) => a + b, 0);
    return nEnemy > oEnemy;
  };

  const filter = (
    results: StatsWithSetup[],
    onlyOpt: boolean
  ): StatsWithSetup[] => {
    if (!onlyOpt) return results;
    let last = null;
    let out = [];
    for (const res of results) {
      if (!last || !sameSetup(last, res)) {
        if (last) out.push(last);
        last = res;
      } else if (better(res, last)) last = res;
    }
    if (last) out.push(last);
    return out;
  };
</script>

<b class="row" class:showSetup>
  {#if showSetup}
    <span>Setup</span>
  {/if}
  <span>Start (s)</span>
  <span>Win Rate</span>
  <span />
  <span>Avg Length (s)</span>
  <span>Longest (s)</span>
</b>
{#if resultIndex !== null || resultsFiltered.length === 1}
  {@const result =
    resultIndex === null ? resultsFiltered[0] : resultsFiltered[resultIndex]}
  <div
    class="row"
    class:showSetup
    class:no-hover={resultIndex === null}
    on:click={() => (resultIndex = null)}
  >
    <ResultRow {result} {showSetup} />
  </div>
  <br />
  <ResultGraph {result} />
{:else}
  {#each resultsFiltered as result, index}
    <div class="row" class:showSetup on:click={() => (resultIndex = index)}>
      <ResultRow {result} {showSetup} />
    </div>
  {/each}
{/if}

<style>
  .row {
    width: 100%;
    text-align: right;
    display: grid;
    grid-template-columns: 70px 80px auto 120px 100px;
    gap: 5px;
    align-items: center;
    padding: 2px 0;
  }
  .showSetup {
    grid-template-columns: 70px 70px 80px auto 120px 100px;
  }
  .row:not(.no-hover) {
    cursor: pointer;
  }
  :not(b).row:hover {
    background-color: var(--card-bg-hover);
  }
</style>
