<script lang="ts">
  import type { Combatant } from '../types';

  export let combatant: Combatant;
  export let isEnemy = false;
</script>

{#if combatant.image_name}
  <img
    class="img"
    class:enemy={isEnemy}
    src="/images/{combatant.image_name}.png"
    title={combatant.name}
    alt={combatant.name}
  />
{:else}
  <div class="img" title={combatant.name} />
{/if}

<style>
  .img {
    background-color: var(--villager);
    border-radius: 50%;
    width: 96px;
    height: 96px;
  }
  .img.enemy {
    background-color: var(--bad);
  }
</style>
