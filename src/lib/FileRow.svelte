<script lang="ts">
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher<{ navigate: {route: string} }>();

  export let name: string;
  export let path: string;
  export let size: number | 'not calculated' = 'not calculated';
</script>

<tr on:click={_ => dispatch('navigate', { route: path })}>
  <td>{name}</td>
  <td>{size}</td>
</tr>

<style>
  tr {
    border-bottom: 1px rgb(212, 212, 212) solid;
  }
  
  tr:hover {
    cursor: pointer;
  }

  td {
    padding: 0.5em 0 0 1em;
  }
</style>