<script lang="ts">
  import Table from "../Table.svelte";
  import TableHeader from "../TableHeader.svelte";
  import { orders$ } from "../OrderManager.svelte";
</script>

<Table auto items={[...$orders$.values()]} let:item={order}>
  <svelte:fragment slot="headers">
    <TableHeader>Order</TableHeader>
    <TableHeader>Status</TableHeader>
  </svelte:fragment>
  <tr>
    <td>{order.path}</td>
    <td>{order.status}</td>
  </tr>
</Table>

<style>
  td {
    padding: .5em 0;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }
</style>