<script lang="ts">
  import { startup$ } from "../SettingsManager.svelte";
  import Icon from "@iconify/svelte";
  import AddIcon from '@iconify/icons-mdi/add';
  import RemoveIcon from '@iconify/icons-mdi/minus';

  let selected: string | null = null;
</script>

<h2>Scan on startup</h2>
<div id="startup">
  <ul>
    {#each $startup$ as startup}
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
    <li class:selected={selected == startup} on:click={_ => selected = startup}>
      {startup}
    </li>
    {/each}
  </ul>
  <div class="side">
    <button><Icon icon={AddIcon} /></button>
    <button><Icon icon={RemoveIcon} /></button>
  </div>
</div>

<style>
  #startup {
    display: flex;
    flex-direction: row;
    width: 100%;
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
  }
</style>