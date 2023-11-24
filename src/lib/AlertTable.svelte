<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import type { Alert } from "./model/alert";
  import Table from "./Table.svelte";
  import TableHeader from "./TableHeader.svelte";
  import Icon from "@iconify/svelte";
  import IconRemove from "@iconify/icons-mdi/remove-box-outline";

  const dispatch = createEventDispatcher<{ select: Alert, remove: Alert }>();

  export let alerts: Alert[];
</script>

{#if alerts.length}
  <Table items={alerts} let:item={alert}>
    <svelte:fragment slot="headers">
      <TableHeader>Items</TableHeader>
      <TableHeader>Size</TableHeader>
      <TableHeader></TableHeader>
    </svelte:fragment>
    <tr on:click={_ => dispatch("select", alert)} class="row">
      <td>
        {alert.filter.item}
      </td>
      <td>
        {alert.filter.minSize} {alert.filter.sizeUnit.symbol}
      </td>
      <td class="remove" on:click={_ => dispatch("remove", alert)}>
        <Icon icon={IconRemove} />
      </td>
    </tr>
  </Table>
{:else}
  <h3>No alerts</h3>
{/if}

<style>
  .row:hover {
    cursor: pointer;
    text-decoration: underline;
  }

  .remove {
    font-size: 1.5em;
    transition: 300ms all ease;
  }

  .remove:hover {
    color: var(--item-active-color);
    transition: 150ms all ease;
  }
</style>