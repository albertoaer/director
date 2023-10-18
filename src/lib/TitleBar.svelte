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
    <button on:mousedown|stopPropagation on:click={() => appWindow.minimize()}>
      <Icon icon={minimizeIcon} color="var(--font-color)" />
    </button>
    <button on:mousedown|stopPropagation on:click={() => appWindow.toggleMaximize()}>
      <Icon icon={maximizeIcon} color="var(--font-color)" />
    </button>
    <button on:mousedown|stopPropagation on:click={() => appWindow.hide()}>
      <Icon icon={closeIcon} color="var(--font-color)" />
    </button>
  </div>
</header>

<style>
  header {
    background-color: var(--window-bar-color);
    border: 0;
    user-select: none;
    display: flex;
    align-items: stretch;
    justify-content: space-between;
    height: 30px;
    position: relative;
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
    background-color: inherit;
    transition: 100ms all ease;
  }
  
  button:hover {
    background-color: var(--window-bar-hover-color);
    cursor: pointer;
    transition: 100ms all ease;
  }
  
  button:active {
    background-color: inherit;
    transition: 100ms all ease;
  }
</style>