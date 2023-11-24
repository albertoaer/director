<script lang="ts" generics="T">
  import Icon from "@iconify/svelte";
  import AddIcon from '@iconify/icons-mdi/add';
  import RemoveIcon from '@iconify/icons-mdi/minus';
  import Button from "./Button.svelte";
  import { createEventDispatcher } from "svelte";

  export let items: T[];

  const dispatch = createEventDispatcher<{ add: void, remove: T }>();

  let selected: T | null = null;
</script>

<div id="container">
  <ul>
    {#each items as item}
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
    <li class:selected={selected == item} on:click={_ => selected = item}>
      {item}
    </li>
    {/each}
  </ul>
  <div class="side">
    <Button on:click={_ => dispatch('add')}><Icon icon={AddIcon} /></Button>
    <Button on:click={_ => { if (selected) dispatch('remove', selected) }}><Icon icon={RemoveIcon} /></Button>
  </div>
</div>

<style>
  #container {
    display: flex;
    flex-direction: row;
    width: 100%;
    gap: 1em;
  }

  ul {
    flex-grow: 1;
    list-style: none;
    margin: 0;
    padding: 0;
  }

  li {
    padding: 2px 4px;
    cursor: pointer;
  }

  li:not(.selected):hover {
    color: var(--item-active-color);
  }

  li.selected {
    background-color: var(--item-active-color);
  }

  .side {
    display: flex;
    flex-direction: column;
    flex-grow: 0;
    gap: 5px;
    font-size: 1.3em;
  }
</style>