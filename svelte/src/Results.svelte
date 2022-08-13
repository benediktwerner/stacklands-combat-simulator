<script lang="ts">
  import ResultGraph from './ResultGraph.svelte';
  import type { StatsWithSetup } from './worker';

  export let results: StatsWithSetup | StatsWithSetup[];

  let resultIndex = null;

  export const reset = () => {
    resultIndex = null;
  };
</script>

{#if !Array.isArray(results)}
  <ResultGraph result={results} />
{:else if resultIndex !== null}
  <ResultGraph result={results[resultIndex]} />
{:else}
  <div class="table">
    <div class="heading start">Start (s)</div>
    <div class="heading win-rate">Win Rate</div>
    <div class="heading win-rate-bar" />
    <div class="heading avg-length">Avg Length (s)</div>
    <div class="heading longest">Longest (s)</div>
    {#each results as result}
      {@const winRate = ((100 * result.wins) / result.iters).toFixed(2)}
      <div class="start">{result.monthStart}</div>
      <div class="win-rate">
        {winRate} %
      </div>
      <div class="win-rate-bar">
        <div class="win-rate-bar-inner" style:width="{winRate}%" />
      </div>
      <div class="avg-length">
        {Math.floor(result.total_length / result.iters)}
      </div>
      <div class="longest">{result.longest}</div>
    {/each}
  </div>
{/if}

<style>
  .table {
    width: 100%;
    text-align: right;
    display: grid;
    grid-template-columns: 70px 80px auto 120px 100px;
    grid-auto-flow: row dense;
    gap: 5px;
    align-items: center;
  }

  .heading {
    font-weight: bold;
  }

  .win-rate-bar:not(.heading) {
    width: 100%;
    height: 1em;
  }
  .win-rate-bar-inner {
    height: 100%;
    background-color: var(--good);
  }
</style>
