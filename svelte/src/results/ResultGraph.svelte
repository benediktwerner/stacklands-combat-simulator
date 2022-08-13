<script lang="ts">
  import type { Stats } from '../wasm/stacklands_combat_simulator';

  export let result: Stats;
  $: maxProb = Math.max(...result.enemy_hp, ...result.village_survivors);
  $: totalProb = [...result.enemy_hp, ...result.village_survivors].reduce(
    (a, b) => a + b,
    0
  );

  let tooltip: HTMLElement;

  const tooltipMove = (e: MouseEvent) => {
    let target = e.target as HTMLElement;
    if (target && !target.dataset.title) target = target.parentElement;
    if (!target || !target.dataset.title) {
      tooltip.classList.add('hidden');
      return;
    }
    tooltip.classList.remove('hidden');
    tooltip.innerText = target.dataset.title;
    tooltip.style.top = e.clientY - 25 + 'px';
    tooltip.style.left = e.clientX + 'px';
  };
</script>

<b>Lefover Enemy HP</b>
<div
  class="stats-graph"
  on:mousemove={tooltipMove}
  on:mouseleave={() => tooltip.classList.add('hidden')}
>
  {#each result.enemy_hp as hp, index}
    <div class="label">
      {90 - index * 10}-{100 - index * 10}%
    </div>
    <div class="bar" data-title="{((100 * hp) / totalProb).toFixed(2)} %">
      <div class="demon-hp" style="width: {(100 * hp) / maxProb}%" />
    </div>
  {/each}
</div>

<br />

<b>Leftover Villagers</b>
<div
  class="stats-graph"
  on:mousemove={tooltipMove}
  on:mouseleave={() => tooltip.classList.add('hidden')}
>
  {#each result.village_survivors as chance, index}
    <div class="label">
      {index + 1}
    </div>
    <div class="bar" data-title="{((100 * chance) / totalProb).toFixed(2)} %">
      <div class="villagers" style="width: {(100 * chance) / maxProb}%" />
    </div>
  {/each}
</div>

<div class="tooltip hidden" bind:this={tooltip} />

<style>
  .demon-hp {
    background-color: var(--bad);
    height: 12px;
  }
  .villagers {
    background-color: var(--good);
    height: 12px;
  }

  .stats-graph {
    display: grid;
    grid-template-columns: 70px auto;
    grid-auto-flow: row dense;
  }
  .stats-graph .label {
    grid-column-start: 1;
    text-align: right;
    padding-right: 7px;
  }
  .stats-graph .bar {
    grid-column-start: 2;
    display: flex;
    align-items: center;
  }
  .stats-graph .bar:hover {
    background-color: var(--card-bg-hover);
  }
  .stats-graph .bar > div {
    transition: width 250ms ease-in-out;
  }

  .tooltip {
    position: fixed;
    background-color: black;
    border-radius: 12px;
    padding: 5px 10px;
    pointer-events: none;
    transform: translateX(-100%);
  }
</style>
