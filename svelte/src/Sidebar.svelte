<script lang="ts">
  import SliderWithValue from './SliderWithValue.svelte';

  export let monthLength: number;
  export let monthStart: number;
  export let iterations: number;
  export let run: () => any;
</script>

<h3>Settings</h3>

<table>
  <tr>
    <td> Month Length: </td>
    <td>
      <select bind:value={monthLength}>
        <option value={90}>Short</option>
        <option value={120}>Normal</option>
        <option value={200}>Long</option>
      </select>
    </td>
  </tr>
  <tr>
    <td> Month Start: </td>
    <td class="center">
      <SliderWithValue bind:value={monthStart} max={monthLength} />
    </td>
  </tr>
  <tr>
    <td> Iterations: </td>
    <td>
      <input type="number" min="1" max="100000000" bind:value={iterations} />
    </td>
  </tr>
</table>

<div class="button-container">
  <button on:click={run}>Simulate</button>
</div>

<style>
  h3 {
    margin-top: 0;
  }

  table,
  tr {
    width: 100%;
  }
  td:nth-child(2) {
    text-align: right;
  }

  .button-container {
    margin-top: 24px;
    display: flex;
    justify-content: center;
  }

  button {
    border-radius: 20px;
    padding: 0 25px;
    line-height: 38px;
    font-size: 14px;
    border: none;
    color: var(--text);
    background-color: #646cff;
    font-family: 'Open Sans';
    font-weight: 600;
    cursor: pointer;
    transition: background-color 250ms;
  }
  button:hover {
    background-color: #747bff;
  }
</style>
