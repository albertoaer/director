<script lang="ts">
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher<{ navigate: {route: string} }>();

  export let name: string;
  export let path: string;
  export let size: number | 'not calculated' = 'not calculated';
</script>

<tr>
  <td><button on:click={_ => dispatch('navigate', { route: path })}>{name}</button></td>
  <td>{size}</td>
</tr>