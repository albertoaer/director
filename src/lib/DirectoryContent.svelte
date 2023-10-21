<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import DirRow from "./DirRow.svelte";
  import type { FSChild } from "./model/fs";
  import { Units, formatBytes } from "./model/units";
  import Table from "./Table.svelte";
  import TableHeader from "./TableHeader.svelte";
  import ContextMenu from "./ContextMenu.svelte";
  import ContextMenuItem from "./ContextMenuItem.svelte";
  import NavigateIcon from '@iconify/icons-mdi/chevron-right';
    import { navigate } from "./FSManager.svelte";

  const dispatch = createEventDispatcher<{ navigate: { route: string } }>();

  export let childs: FSChild[];

  let sizeUnitsIdx = 0;
  function updateSizeUnit() {
    sizeUnitsIdx = (sizeUnitsIdx + 1) % Units.length;
  }
</script>

<ContextMenu let:contextMenu>
  <svelte:fragment slot="menu" let:payload>
    {#if payload}
      <ContextMenuItem icon={NavigateIcon} on:click={_ => navigate(payload.id)}>navigate</ContextMenuItem>
    {/if}
  </svelte:fragment>
  <Table>
    <svelte:fragment slot="headers">
      <TableHeader>Name</TableHeader>
      <TableHeader on:click={updateSizeUnit} action>Size ({Units[sizeUnitsIdx].symbol})</TableHeader>
      <TableHeader>Modified</TableHeader>
      <TableHeader>Created</TableHeader>
    </svelte:fragment>
    {#each childs as child (child.path)}
      <DirRow
        {contextMenu}
        {child}
        mapSize={size => formatBytes(size, Units[sizeUnitsIdx])}
        on:navigate={event => dispatch("navigate", { route: event.detail.route })}
      />
    {/each}
  </Table>
</ContextMenu>