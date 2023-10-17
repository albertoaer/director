<script lang="ts">
  import type { ComponentProps, ComponentType } from "svelte";
  import NavBar from "./NavBar.svelte";

  export let items: ComponentProps<NavBar>['items'];
  
  let component: ComponentType | undefined = undefined;
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
    padding: 0.5em;
    width: 100%;
    height: 100%;
  }
</style>