<script lang="ts">
  import Table from "../Table.svelte";
  import TableHeader from "../TableHeader.svelte";
  import { orders$ } from "../OrderManager.svelte";
  import Dots from "../Dots.svelte";
</script>

<Table items={[...$orders$.values()]} let:item={order}>
  <svelte:fragment slot="headers">
    <TableHeader width='70%'>Order</TableHeader>
    <TableHeader>Status</TableHeader>
  </svelte:fragment>
  <tr>
    <td>{order.path}</td>
    <td>{order.status}<Dots cond={order.status === 'Running'}/></td>
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