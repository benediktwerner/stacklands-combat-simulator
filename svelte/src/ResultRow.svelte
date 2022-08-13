<script lang="ts">
  import type { StatsWithSetup } from './worker';

  export let result: StatsWithSetup;

  $: winRate = ((100 * result.wins) / result.iters).toFixed(2);
</script>

<div>{result.monthStart}</div>
<div>
  {winRate} %
</div>
<div class="win-rate-bar">
  <div class="win-rate-bar-inner" style:width="{winRate}%" />
</div>
<div>
  {Math.floor(result.total_length / result.iters)}
</div>
<div>{result.longest}</div>

<style>
  .win-rate-bar:not(.heading) {
    width: 100%;
    height: 1em;
  }
  .win-rate-bar-inner {
    height: 100%;
    background-color: var(--good);
  }
</style>
