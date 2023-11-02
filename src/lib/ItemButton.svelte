<script lang="ts">
  import { tooltip as tooltipFn } from "./Tooltip.svelte";

  export let tooltip: Parameters<typeof tooltipFn>[1] = undefined;
</script>

<button on:click|stopPropagation use:tooltipFn={tooltip}>
  <slot />
</button>

<style>
  button {
    margin: 0;
    padding: 0.25em 0.5em;
    border: none;
    background-color: inherit;
    color: inherit;
    height: 100%;
    font-size: inherit;
  }
  
  button:hover {
    cursor: pointer;
    background-color: var(--item-active-color);
    transition: 200ms all ease;
    font-size: inherit;
  }
</style>