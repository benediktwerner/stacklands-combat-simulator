<script lang="ts">
  export let value: number;
  export let max: number;
  export let min: number = 0;
  export let step: number = 1;
  export let showMax: boolean = false;
</script>

<div>
  <span>
    {value}{#if showMax}/{max}{/if}
  </span>
  <input type="range" {min} {step} {max} bind:value />
</div>

<style>
  div {
    display: flex;
    flex-direction: column;
    text-align: center;
    justify-content: center;
  }
  input {
    width: 100%;
  }
  span {
    margin-bottom: -10px;
  }
</style>
