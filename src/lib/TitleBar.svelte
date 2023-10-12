<script lang="ts">
  import { getName } from '@tauri-apps/api/app';
  import { appWindow } from '@tauri-apps/api/window';
  import Icon from './Icon.svelte';
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<header on:mousedown={() => appWindow.startDragging()}>
  <h2>
    {#await getName() then name}
      {name}
    {/await}
  </h2>
  <div id="actions">
    <!-- svelte-ignore a11y-missing-attribute -->
    <button on:mousedown|stopPropagation on:click={() => appWindow.minimize()}><Icon id="mdi:window-minimize" /></button>
    <button on:mousedown|stopPropagation on:click={() => appWindow.toggleMaximize()}><Icon id="mdi:window-maximize" /></button>
    <button on:mousedown|stopPropagation on:click={() => appWindow.hide()}><Icon id="mdi:window-close" /></button>
  </div>
</header>

<style>
  h2 {
    margin: 0;
    padding: 0.2em;
  }

  header {
    background-color: rgb(160, 160, 209);
    border: 0;
    user-select: none;
    display: flex;
    align-items: stretch;
    justify-content: space-between;
  }

  #actions {
    display: flex;
  }

  button {
    border: none;
    margin: 0;
    padding: 0.5em 1em;
    background-color: rgb(160, 160, 209);
    transition: 100ms all ease;
  }
  
  button:hover {
    background-color: rgb(170, 170, 170);
    cursor: pointer;
    transition: 100ms all ease;
  }
  
  button:active {
    background-color: rgb(134, 134, 134);
    transition: 100ms all ease;
  }
</style>