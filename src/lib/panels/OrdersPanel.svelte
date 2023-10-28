<script lang="ts">
  import Table from "../Table.svelte";
  import TableHeader from "../TableHeader.svelte";
  import { orders$ } from "../OrderManager.svelte";
  import Icon from "@iconify/svelte";
</script>

<Table auto>
  <svelte:fragment slot="headers">
    <TableHeader>Order</TableHeader>
    <TableHeader>Status</TableHeader>
  </svelte:fragment>
  {#each $orders$ as order}
    <tr>
      <td>{order[1].path}</td>
      <td>{order[1].status}</td>
    </tr>
  {/each}
</Table>

<style>
  td {
    padding: .5em 0;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }
</style>