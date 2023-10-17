<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import type { FSChild } from "./model";

  import Icon from "@iconify/svelte";
  import FileIcon from "@iconify/icons-mdi/file-outline";
  import DirectoryIcon from "@iconify/icons-mdi/folder-outline";
  import LinkIcon from "@iconify/icons-mdi/link";
  import OtherIcon from "@iconify/icons-mdi/question-mark";
  import UndefinedIcon from "@iconify/icons-mdi/error";

  const dispatch = createEventDispatcher<{ navigate: {route: string} }>();

  export let child: FSChild;

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

<tr on:click={click}>
  <td class="name" class:nav={canNavigate(child.type)} ><Icon icon={getIcon(child.type)} />{child.name}</td>
  <td class:dots={child.size.status === 'Calculating'}>{child.size.value ?? child.size.status}</td>
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

  tr:hover {
    border-bottom: 1px rgb(212, 212, 212) solid;
    cursor: pointer;
  }

  td {
    padding: .5em 0;
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

  @keyframes dots {
    0%   { content: '.  ';   }
    50%  { content: '.. ';  }
    100% { content: '...'; }
  }

  td.dots::after {
    content: '';
    animation: dots 1s infinite linear;
    animation-delay: 0;
  }
</style>