<script lang="ts">
  import type { CombatantSetup } from 'src/types';

  import type { StatsWithSetup } from '../worker';

  export let result: StatsWithSetup;
  export let showSetup: boolean;

  $: winRate = ((100 * result.wins) / result.iters).toFixed(2);

  const variedSetup = (setup: CombatantSetup[]) => {
    const result = [];
    for (const c of setup) {
      if (c.vary) result.push(c);
    }
    return result;
  };
</script>

{#if showSetup}
  {@const vills = variedSetup(result.villagerSetup)}
  {@const enemies = variedSetup(result.enemySetup)}
  <div>
    {#if vills.length > 0}
      {#each vills as c, i}
        <span title={c.name}>{c.count}</span>
        {#if i !== vills.length - 1}/{/if}
      {/each}
    {/if}
    {#if vills.length > 0 && enemies.length > 0}|{/if}
    {#if enemies.length > 0}
      {#each enemies as c, i}
        <span title={c.name}>{c.count}</span>
        {#if i !== enemies.length - 1}/{/if}
      {/each}
    {/if}
  </div>
{/if}
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
