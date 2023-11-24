<script lang="ts">
  import { confirm } from "@tauri-apps/api/dialog";
  import { createEventDispatcher } from "svelte";
  import Button from "./Button.svelte";
  import { Units, convertToBytes } from "./model/units";
  import { AlertItems, type AlertItem, type Alert } from "./model/alert";

  const dispatch = createEventDispatcher<{ produce: Alert }>();

  export let alert: Alert | undefined = undefined;

  let item: AlertItem = 'any';
  let minSize: number = 0;
  let sizeUnitSymbol: string = Units[0].symbol;

  $: if (alert) {
    item = alert.filter.item;
    minSize = alert.filter.minSize;
    sizeUnitSymbol = alert.filter.sizeUnit.symbol;
    alert = undefined;
  }

  async function apply() {
    let sizeUnit = Units.find(x => x.symbol == sizeUnitSymbol)!;
    let mbReference = Units.find(x => x.symbol == 'MB')!;
    if (
      convertToBytes(minSize, sizeUnit.factor) >= Math.pow(10, mbReference.factor) ||
      await confirm(
        "Is not recomendable to use a filter with a min size of less than 1MB, Proceed?",
        "Recommendation"
      )
    ) {
      dispatch("produce", {
        name: item,
        filter: {
          item,
          minSize,
          sizeUnit
        }
      });
    }
  }
</script>

<form on:submit|preventDefault>
  <h1>Configure alert</h1>
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
  <label for="name">Size</label>
  <div class="row">
    <input class="row-main" type="number" name="size" id="size" bind:value={minSize} min="0">
    <select name="unit" bind:value={sizeUnitSymbol}>
      {#each Units as unit}
        <option value={unit.symbol}>
          {unit.symbol}
        </option>
      {/each}
    </select>
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