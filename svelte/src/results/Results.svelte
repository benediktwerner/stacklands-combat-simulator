<script lang="ts">
  import type { StatsWithSetup } from '../worker';
  import ResultGraph from './ResultGraph.svelte';
  import ResultRow from './ResultRow.svelte';

  export let results: StatsWithSetup[];

  let resultIndex: number | null = null;

  $: showSetup =
    results.length > 0 &&
    (results[0].villagerSetup.some((c) => c.vary) ||
      results[0].enemySetup.some((c) => c.vary));

  export const reset = () => {
    resultIndex = null;
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
{#if resultIndex !== null || results.length === 1}
  {@const result = resultIndex === null ? results[0] : results[resultIndex]}
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
  {#each results as result, index}
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
