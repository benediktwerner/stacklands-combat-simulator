<script lang="ts">
  export let onlyShowOptimal: boolean;
</script>

<h3>Display Settings</h3>

<table>
  <tr>
    <td>Only show optimal start time:</td>
    <td>
      <input type="checkbox" bind:checked={onlyShowOptimal} />
    </td>
  </tr>
</table>

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
</style>
