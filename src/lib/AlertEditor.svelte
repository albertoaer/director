<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import Button from "./Button.svelte";
  import { Units, type Unit } from "./model/units";
  import { AlertItems, type AlertItem, type Alert } from "./model/alert";

  const dispatch = createEventDispatcher<{ produce: Alert }>();

  export let alert: Alert | undefined = undefined;

  let name: string = '';
  let item: AlertItem = 'all';
  let minSize: number = 0;
  let sizeUnit: Unit = Units[0];

  $: if (alert) {
    name = alert.name;
    item = alert.filter.item;
    minSize = alert.filter.minSize;
    sizeUnit = alert.filter.sizeUnit;
    alert = undefined;
  }

  function apply() {
    dispatch("produce", {
      name,
      filter: {
        item,
        minSize,
        sizeUnit
      }
    });
  }
</script>

<form on:submit|preventDefault>
  <h1>Configure alert</h1>
  <label for="name">Name</label>
  <input type="text" name="name" id="name" bind:value={name}>
  <label for="name">Size</label>
  <div class="row">
    <input class="row-main" type="number" name="size" id="size" bind:value={minSize}>
    <select name="unit" bind:value={sizeUnit}>
      {#each Units as unit}
        <option
          value={unit}
          selected={sizeUnit.symbol == unit.symbol}
        >
          {unit.symbol}
        </option>
      {/each}
    </select>
  </div>
  <div class="row">
    {#each AlertItems as alertItem}
      <div>
        <input type="radio" name="only"
          value={alertItem}
          id="{alertItem}_input"
          checked={item === alertItem}
          on:change={_ => item = alertItem}
        >
        <label for="{alertItem}_input">{alertItem}</label>
      </div>
    {/each}
  </div>
  <Button grow on:click={apply}>Apply</Button>
</form>

<style>
  form {
    display: flex;
    flex-direction: column;
    width: 80%;
    margin: auto;
    gap: 1em;
    align-items: stretch;
  }

  form > label, form > div {
    text-align: left;
  }
  
  input, select {
    font-size: 1.5em;
    background-color: var(--item-color);
    margin: 0;
    padding: 0.3em;
    outline: none;
    border: none;
    border-radius: 5px;
    color: inherit;
    text-align: left;
    min-width: 0;
  }
  
  .row {
    display: flex;
  }

  .row > * {
    flex: 1 1 0;
  }
  
  .row > .row-main {
    flex: 3 0 0;
  }
</style>