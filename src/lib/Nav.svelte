<script lang="ts">
  import type { ComponentProps, ComponentType } from "svelte";
  import NavBar from "./NavBar.svelte";

  export let items: ComponentProps<NavBar>['items'];
  
  let component: ComponentType | undefined = items[0]?.component;
</script>

<div id="main">
  <NavBar {items} on:selected={ev => component = ev.detail.component}></NavBar>
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
    height: 100%;
  }
</style>