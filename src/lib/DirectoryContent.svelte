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
  import InfoIcon from '@iconify/icons-mdi/info-box';
  import NavigateIcon from '@iconify/icons-mdi/chevron-right';
  import CalculateIcon from '@iconify/icons-mdi/scale-unbalanced';
  import AddStartupIcon from '@iconify/icons-mdi/calendar-add';
  import RemoveStartupIcon from '@iconify/icons-mdi/calendar-minus';
  import { navigate } from "./FSManager.svelte";
  import { addStartup, removeStartup } from "./SettingsManager.svelte";

  const dispatch = createEventDispatcher<{ navigate: { route: string } }>();

  export let childs: FSChild[];
</script>

<ContextMenu {childs} let:contextMenu>
  <svelte:fragment slot="menu" let:payload>
    <ContextMenuItem icon={InfoIcon}>
      { payload.type } : { payload.size.status } size
    </ContextMenuItem>
    {#if payload.type === 'directory'}
      <ContextMenuItem icon={NavigateIcon} on:click={_ => navigate(payload.path)}>
        navigate
      </ContextMenuItem>
      <ContextMenuItem icon={CalculateIcon} on:click={_ => navigate(payload.path, { calculate: true })}>
        navigate & calculate
      </ContextMenuItem>
      <ContextMenuItem icon={AddStartupIcon} on:click={_ => addStartup(payload.path)}>
        add to startup
      </ContextMenuItem>
      <ContextMenuItem icon={RemoveStartupIcon} on:click={_ => removeStartup(payload.path)}>
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