<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { routeToString, type Route } from "./model";

  const dispatch = createEventDispatcher<{ navigate: {route: string} }>();

  export let route: Route;

  let value = '';
  $: value = routeToString(route);
  
  let editValue = '';

  let editMode = false;
  let timeout: NodeJS.Timeout | undefined;

  const DOUBLE_CLICK_TIMEOUT = 250;

  function handleItemClick(event: MouseEvent, route: string) {
    if (event.detail == 1) {
      timeout = setTimeout(() => submit(route), DOUBLE_CLICK_TIMEOUT);
    }
    if (event.detail == 2) {
      editMode = true;
      clearTimeout(timeout);
    }
  }

  function setupInput(component: HTMLInputElement) {
    editValue = value;
    component.focus();
  }

  function submit(route: string) {
    editMode = false;
    dispatch('navigate', { route })
  }
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<!-- svelte-ignore a11y-click-events-have-key-events -->
<div id="bar" on:click={_ => editMode = true}>
  {#if editMode}
    <form on:submit|preventDefault={_ => submit(editValue)}>
      <input use:setupInput bind:value={editValue} type="text" on:blur={_ => editMode = false}>
    </form>
  {:else}
    <div>
      {#each route.items as item, i}
        <button on:click|stopPropagation={event => handleItemClick(event, item.path)}>
          {item.name}
        </button>
      {/each}
    </div>
  {/if}
</div>

<style>
  #bar {
    font-size: 1.2em;
  }

  input {
    margin: 0;
    padding: 0;
    border: none;
    outline: 0;
    display: inline-flex;
    width: 100%;
    height: 100%;
    font-size: inherit;
  }

  button {
    margin: 0;
    padding: 0 0.5em;
    border: none;
    background-color: rgb(200, 200, 200);
    height: 100%;
    font-size: inherit;
  }
  
  button:hover {
    cursor: pointer;
    background-color: rgb(168, 168, 168);
    transition: 200ms all ease;
    font-size: inherit;
  }
</style>