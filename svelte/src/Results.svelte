<script lang="ts">
  import ResultGraph from './ResultGraph.svelte';
  import ResultRow from './ResultRow.svelte';
  import type { StatsWithSetup } from './worker';

  export let results: StatsWithSetup[];

  let resultIndex = null;

  export const reset = () => {
    resultIndex = null;
  };
</script>

<b class="row">
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
    class:no-hover={resultIndex === null}
    on:click={() => (resultIndex = null)}
  >
    <ResultRow {result} />
  </div>
  <br />
  <ResultGraph {result} />
{:else}
  {#each results as result, index}
    <div class="row" on:click={() => (resultIndex = index)}>
      <ResultRow {result} />
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
  .row:not(.no-hover) {
    cursor: pointer;
  }
  :not(b).row:hover {
    background-color: var(--card-bg-hover);
  }
</style>
