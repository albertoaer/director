<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import type { FSChild } from "./model/fs";

  import Icon from "@iconify/svelte";
  import FileIcon from "@iconify/icons-mdi/file-outline";
  import DirectoryIcon from "@iconify/icons-mdi/folder-outline";
  import LinkIcon from "@iconify/icons-mdi/link";
  import OtherIcon from "@iconify/icons-mdi/question-mark";
  import UndefinedIcon from "@iconify/icons-mdi/error";
  import tippy from "tippy.js";
  import Dots from "./Dots.svelte";
  import type { ContextMenuEvent } from "./ContextMenu.svelte";

  const dispatch = createEventDispatcher<{ navigate: {route: string} }>();

  export let child: FSChild;
  export let mapSize: (size: number) => string = size => size.toString();
  export let contextMenu: ContextMenuEvent<FSChild> | undefined = undefined;

  function getIcon(type: typeof child.type) {
    switch (type) {
      case "file": return FileIcon;
      case "directory": return DirectoryIcon;
      case "link": return LinkIcon;
      case "other": return OtherIcon;
      default: return UndefinedIcon;
    }
  }

  function canNavigate(type: typeof child.type) {
    return type === 'directory';
  }

  function click() {
    if (!canNavigate(child.type)) return;
    dispatch('navigate', { route: child.path });
  }
</script>

<tr use:tippy={{ content: child.path }} on:click={click} on:contextmenu={ev => contextMenu?.(child, ev.clientX, ev.clientY, ev)}>
  <td class="name" class:nav={canNavigate(child.type)} ><Icon icon={getIcon(child.type)} />{child.name}</td>
  <td>
    {child.size.value !== undefined ? mapSize(child.size.value) : child.size.status}
    <Dots cond={child.size.status === 'Calculating'}/>
  </td>
  {#if child.modified}
    <td>{new Date(child.modified).toLocaleString()}</td>
  {:else}
    <td>Unknown</td>
  {/if}
  {#if child.created}
    <td>{new Date(child.created).toLocaleString()}</td>
  {:else}
    <td>Unknown</td>
  {/if}
</tr>

<style>
  tr {
    border-bottom: 1px transparent solid;
  }

  tr:hover, tr:active {
    border-bottom: 1px var(--item-active-color) solid;
    cursor: pointer;
  }

  td {
    padding: .5em 0.5em;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }
  
  td.name {
    display: flex;
    align-items: center;
    gap: 0.5em;
  }

  td.nav {
    font-style: italic;
  }
</style>