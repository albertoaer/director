<script lang="ts" context="module">
  import { writable } from "svelte/store";

  const maxFontSize = 1.8;
  const minFontSize = 0.6;

  let fontSize = writable((maxFontSize + minFontSize) / 2);

  function updateFontSize(value: number, allow: boolean) {
    if (value == 0 || !allow) return;
    const mov = value / -Math.abs(value);
    fontSize.update(value => Math.min(Math.max(minFontSize, value + (mov * 0.2)), maxFontSize));
  }
</script>

<script>
  export let auto = false;
</script>

<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<table
  on:wheel={ev => updateFontSize(ev.deltaY, ev.ctrlKey)}
  style="--font-size: {$fontSize}em"
  class:auto
>
  <tr>
    <slot name="headers" />
  </tr>
  <slot />
</table>

<style>
  table {
    width: 100%;
    text-align: left;
    border-spacing: 0px;
    border-collapse: collapse;
    table-layout: fixed;
    font-size: var(--font-size);
  }

  table.auto {
    table-layout: auto;
  }

  tr {
    position: sticky;
    background-color: var(--background-color);
    top: 0;
  }
</style>