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
        <Icon icon={item.icon} color="var(--font-color)" />
      </button>
    </li>
  {/each}
</ul>

<style>
  #bar {
    background-color: var(--nav-bar-color);
    margin: 0;
    padding: 0;
    box-shadow: 0px 0px 7px 0px black;
    overflow: hidden;
  }

  button {
    background-color: inherit;
    border: none;
    padding: 0;
    cursor: pointer;
    padding: 0.25em 0.5em;
    display: flex;
    font-size: 2em;
    justify-content: center;
    transition: 200ms background-color ease;
  }
  
  li:hover button {
    background-color: var(--nav-bar-hover-color);
    transition: 200ms background-color ease-out;
  }
</style>