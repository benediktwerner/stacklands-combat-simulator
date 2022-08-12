<script lang="ts">
  export let value: number;
  export let max: number;
</script>

<div>
  <span>{value}/{max}</span>
  <input type="range" min="0" {max} bind:value />
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
