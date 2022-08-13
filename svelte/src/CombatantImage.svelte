<script lang="ts">
  import type { Combatant } from './types';

  export let combatant: Combatant;
  export let isEnemy = false;
</script>

<object
  class:enemy={isEnemy}
  data="/images/{combatant.name}.png"
  title={combatant.name}
  type="image/png"
  width="96"
  height="96"
/>

<style>
  object {
    background-color: var(--villager);
    border-radius: 50%;
  }
  object.enemy {
    background-color: var(--bad);
  }
</style>
