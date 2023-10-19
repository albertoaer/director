<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import type { Alarm } from "./model/alarm";
  import Table from "./Table.svelte";
  import TableHeader from "./TableHeader.svelte";
  import Icon from "@iconify/svelte";
  import IconRemove from "@iconify/icons-mdi/remove-box-outline";

  const dispatch = createEventDispatcher<{ select: Alarm, remove: Alarm }>();

  export let alarms: Alarm[];
</script>

{#if alarms.length}
  <Table>
    <svelte:fragment slot="headers">
      <TableHeader>Name</TableHeader>
      <TableHeader>Size</TableHeader>
      <TableHeader>Items</TableHeader>
      <TableHeader></TableHeader>
    </svelte:fragment>
    {#each alarms as alarm}
      <tr on:click={_ => dispatch("select", alarm)} class="row">
        <td>
          {alarm.name}
        </td>
        <td>
          {alarm.filter.minSize} {alarm.filter.sizeUnit.symbol}
        </td>
        <td>
          {alarm.filter.item}
        </td>
        <td class="remove" on:click={_ => dispatch("remove", alarm)}>
          <Icon icon={IconRemove} />
        </td>
      </tr>
    {/each}
  </Table>
{:else}
  <h3>No alarms</h3>
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