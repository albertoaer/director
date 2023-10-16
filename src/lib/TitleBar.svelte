<script lang="ts">
  import { getName } from '@tauri-apps/api/app';
  import { appWindow } from '@tauri-apps/api/window';
  import Icon from '@iconify/svelte';

  import minimizeIcon from '@iconify/icons-mdi/window-minimize';
  import maximizeIcon from '@iconify/icons-mdi/window-maximize';
  import closeIcon from '@iconify/icons-mdi/window-close';
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<header on:mousedown={() => appWindow.startDragging()}>
  <p>
    {#await getName() then name}
      {name}
    {/await}
  </p>
  <div id="actions">
    <!-- svelte-ignore a11y-missing-attribute -->
    <button on:mousedown|stopPropagation on:click={() => appWindow.minimize()}><Icon icon={minimizeIcon} /></button>
    <button on:mousedown|stopPropagation on:click={() => appWindow.toggleMaximize()}><Icon icon={maximizeIcon} /></button>
    <button on:mousedown|stopPropagation on:click={() => appWindow.hide()}><Icon icon={closeIcon} /></button>
  </div>
</header>

<style>
  header {
    background-color: rgb(160, 160, 209);
    border: 0;
    user-select: none;
    display: flex;
    align-items: stretch;
    justify-content: space-between;
    position: sticky;
    top: 0;
    height: 27px;
  }

  p {
    margin: 0;
    padding: 5px;
    font-size: 16px;
    height: 100%;
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