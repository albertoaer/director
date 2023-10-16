<script lang="ts">
  import { createEventDispatcher, type ComponentType } from "svelte";
  import Icon, { type IconifyIcon } from "@iconify/svelte";

  interface NavigationItem {
    name: string,
    component: ComponentType,
    icon: string | IconifyIcon
  }

  const dispatch = createEventDispatcher<{ selected: NavigationItem }>();

  export let items: NavigationItem[];
</script>

<ul id="bar">
  {#each items as item (item.name)}
    <li class="item">
      <button on:click={_ => dispatch('selected', item)}>
        <Icon icon={item.icon} />
      </button>
    </li>
  {/each}
</ul>

<style>
  #bar {
    background-color: rgb(189, 189, 251);
    margin: 0;
    padding: 0;
  }

  button {
    background-color: transparent;
    border: none;
    padding: 0;
    cursor: pointer;
    padding: 0.25em 0.5em;
    display: flex;
    font-size: 2em;
    justify-content: center;
    transition: 200ms transform ease;
  }
  
  li:hover button {
    transform: scale(0.95);
    transition: 150ms transform ease-out;
  }
</style>