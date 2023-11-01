<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { routeToString, type Route } from "./model/fs";
  import { fade } from "svelte/transition";
  import { open } from "@tauri-apps/api/dialog";
  import Icon from "@iconify/svelte";
  import OpenFolderIcon from "@iconify/icons-mdi/folder-open-outline";
  import CalculateFolderIcon from "@iconify/icons-mdi/scale-unbalanced";
  import ChartIcon from "@iconify/icons-mdi/chart-pie";
  import { tooltip } from "./Tooltip.svelte";

  const dispatch = createEventDispatcher<{
    navigate: { route: string },
    calculate: { route: string }
  }>();

  export let route: Route;

  let value = '';
  $: value = routeToString(route);
  
  let editValue = '';

  let editMode = false;
  let timeout: NodeJS.Timeout | undefined;

  const DOUBLE_CLICK_TIMEOUT = 250;

  function handleItemClick(event: MouseEvent, route: string | undefined) {
    if (event.detail == 1 && route) {
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

  async function openFolder() {
    const result = await open({
      directory: true,
      multiple: false,
      recursive: false,
      defaultPath: route.path,
      title: 'Open Directory'
    });
    if (result && typeof result === 'string')
      dispatch('navigate', { route: result });
  }

  function calculateFolder() {
    dispatch('calculate', { route: route.path });
  }
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<!-- svelte-ignore a11y-click-events-have-key-events -->
<div id="bar" on:click|stopPropagation={event => handleItemClick(event, undefined)}>
  {#if editMode}
    <form in:fade={{ duration: 200 }} on:submit|preventDefault={_ => submit(editValue)}>
      <input use:setupInput bind:value={editValue} type="text" on:blur={_ => editMode = false}>
    </form>
  {:else}
    <div id="buttons" in:fade={{ duration: 200 }}>
      <button
        on:click|stopPropagation={calculateFolder}
        use:tooltip={{ content: 'toggle chart', singleton: 'dir-bar' }}
      >
        <Icon icon={ChartIcon} inline />
      </button>
      <button
        on:click|stopPropagation={calculateFolder}
        use:tooltip={{ content: 'calculate folder', singleton: 'dir-bar' }}
      >
        <Icon icon={CalculateFolderIcon} inline />
      </button>
      <vr />
      <button
        on:click|stopPropagation={openFolder}
        use:tooltip={{ content: 'open folder', singleton: 'dir-bar' }}
      >
        <Icon icon={OpenFolderIcon} inline />
      </button>
      {#each route.items as item}
        <button
          on:click|stopPropagation={event => handleItemClick(event, item.path)}
          use:tooltip={{ content: item.path, singleton: 'dir-bar' }}
        >
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
    flex: 1 1 0;
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

  #buttons {
    display: flex;
    flex-wrap: wrap;
  }

  vr {
    width: 2px;
    background-color: var(--font-color);
    margin: 5px;
    user-select: none;
  }
</style>