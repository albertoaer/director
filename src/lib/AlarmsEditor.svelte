<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import Button from "./Button.svelte";
  import { Units, type Unit } from "./model/units";
  import { AlarmItems, type AlarmItem, type Alarm } from "./model/alarm";

  const dispatch = createEventDispatcher<{ produce: Alarm }>();

  export let alarm: Alarm | undefined = undefined;

  let name: string = '';
  let item: AlarmItem = 'all';
  let minSize: number = 0;
  let sizeUnit: Unit = Units[0];

  $: if (alarm) {
    name = alarm.name;
    item = alarm.filter.item;
    minSize = alarm.filter.minSize;
    sizeUnit = alarm.filter.sizeUnit;
    alarm = undefined;
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
  <h1>Configure alarm</h1>
  <label for="name">Name</label>
  <input type="text" name="name" id="name" bind:value={name}>
  <label for="name">Size</label>
  <div class="row">
    <input type="number" name="size" id="size" bind:value={minSize}>
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
    {#each AlarmItems as alarmItem}
      <div>
        <input type="radio" name="only"
          value={alarmItem}
          id="{alarmItem}_input"
          checked={item === alarmItem}
          on:change={_ => item = alarmItem}
        >
        <label for="{alarmItem}_input">{alarmItem}</label>
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
  }
  
  .row {
    display: flex;
    width: 100%;
    justify-content: stretch;
    align-items: stretch;
  }

  .row > * {
    flex-grow: 1;
  }
</style>