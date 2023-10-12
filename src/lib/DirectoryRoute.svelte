<script lang="ts">
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher<{ navigate: {route: string} }>();

  export let route: { name: string, path: string }[];

  let value = '';
  $: value = '/' + route.map(x => x.name).join('/');
  
  let editValue = '';

  let editMode = false;
  let timeout: NodeJS.Timeout | undefined;

  function handleItemClick(event: MouseEvent, route: string) {
    if (event.detail == 1) {
      timeout = setTimeout(() => submit(route), 200);
    }
    if (event.detail == 2) {
      editMode = true;
      clearTimeout(timeout)
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

<div>
  {#if editMode}
  <form on:submit|preventDefault={_ => submit(editValue)}>
    <input use:setupInput bind:value={editValue} type="text" on:blur={_ => editMode = false}>
  </form>
  {:else}
    <button on:click={_ => editMode = true}>
      {#each route as item}
        /
        <button class="sub" on:click|stopPropagation={event => handleItemClick(event, item.path)}>
          {item.name}
        </button>
      {/each}
  </button>
  {/if}
</div>

<style>
  input {
    border: none;
    outline: 1px solid black;
    display: inline-flex;
    width: 100%;
  }

  button {
    margin: 0;
    padding: 0;
    border: none;
    background-color: transparent;
    display: inline-flex;
  }
  
  button.sub:hover {
    cursor: pointer;
    background-color: aquamarine;
    transition: 200ms all ease;
  }
</style>