<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { routeToString, type Route } from "./model";
  import { fade } from "svelte/transition";

  const dispatch = createEventDispatcher<{ navigate: { route: string } }>();

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
    <form in:fade={{ duration: 200 }} on:submit|preventDefault={_ => submit(editValue)}>
      <input use:setupInput bind:value={editValue} type="text" on:blur={_ => editMode = false}>
    </form>
  {:else}
    <div in:fade={{ duration: 200 }}>
      {#each route.items as item}
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
    border-radius: 5px;
    overflow: hidden;
    background-color: var(--item-color);
  }

  input {
    margin: 0;
    padding: 0.25em;
    border: none;
    outline: 0;
    background-color: inherit;
    color: inherit;
    display: inline-flex;
    width: 100%;
    height: 100%;
    font-size: inherit;
  }

  button {
    margin: 0;
    padding: 0.25em 0.5em;
    border: none;
    background-color: inherit;
    color: inherit;
    height: 100%;
    font-size: inherit;
  }
  
  button:hover {
    cursor: pointer;
    background-color: var(--item-active-color);
    transition: 200ms all ease;
    font-size: inherit;
  }
</style>