<script lang="ts" context="module">
  import { writable } from "svelte/store";

  let sizeUnitsIdx = writable(0);

  function updateSizeUnit() {
    sizeUnitsIdx.update(idx => (idx + 1) % Units.length);
  }
</script>

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
  import CalculateIcon from '@iconify/icons-mdi/scale-unbalanced';
  import AddStartupIcon from '@iconify/icons-mdi/calendar-add';
  import RemoveStartupIcon from '@iconify/icons-mdi/calendar-minus';
  import { navigate } from "./FSManager.svelte";
  import { add_startup, remove_startup } from "./SettingsManager.svelte";

  const dispatch = createEventDispatcher<{ navigate: { route: string } }>();

  export let childs: FSChild[];
</script>

<ContextMenu let:contextMenu>
  <svelte:fragment slot="menu" let:payload>
    {#if payload}
      <ContextMenuItem icon={NavigateIcon} on:click={_ => navigate(payload.id)}>
        navigate
      </ContextMenuItem>
      <ContextMenuItem icon={CalculateIcon} on:click={_ => navigate(payload.id, { calculate: true })}>
        navigate & calculate
      </ContextMenuItem>
      <ContextMenuItem icon={AddStartupIcon} on:click={_ => add_startup(payload.id)}>
        add to startup
      </ContextMenuItem>
      <ContextMenuItem icon={RemoveStartupIcon} on:click={_ => remove_startup(payload.id)}>
        remove from startup
      </ContextMenuItem>
    {/if}
  </svelte:fragment>
  <Table items={childs} let:item={child}>
    <svelte:fragment slot="headers">
      <TableHeader width='35%'>Name</TableHeader>
      <TableHeader width='25%' on:click={updateSizeUnit} action>Size ({Units[$sizeUnitsIdx].symbol})</TableHeader>
      <TableHeader width='20%' >Modified</TableHeader>
      <TableHeader width='20%' >Created</TableHeader>
    </svelte:fragment>
    <DirRow
      {contextMenu}
      {child}
      mapSize={size => formatBytes(size, Units[$sizeUnitsIdx])}
      on:navigate={event => dispatch("navigate", { route: event.detail.route })}
    />
  </Table>
</ContextMenu>