<script lang="ts">
  import type { ComponentProps, ComponentType } from "svelte";
  import NavBar, { type NavigationItem } from "./NavBar.svelte";

  export let items: ComponentProps<NavBar>['items'];
  
  let component: ComponentType | undefined = items[0]?.component;
  let selected = items[0]?.name;

  function handleSelection(ev: CustomEvent<NavigationItem>) {
    component = ev.detail.component;
    selected = ev.detail.name;
  }
</script>

<div id="main">
  <NavBar {items} {selected} on:selected={handleSelection}></NavBar>
  <div id="panel">
    <svelte:component this={component} />
  </div>
</div>

<style>
  #main {
    display: flex;
    flex-direction: row;
    align-items: stretch;
    justify-content: stretch;
    height: 100%;
  }

  #panel {
    padding: 0.7em 0.7em 0.7em calc(0.7em + 1vw);
    width: 100%;
    height: calc(100% - 4em);
  }
</style>