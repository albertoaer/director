<script lang="ts" context="module">
  import { writable, type Writable } from "svelte/store";
  import type { Order } from "./model/order";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/tauri";

  export const orders$: Writable<Map<string, Order>> = writable(new Map());

  listen<Order>('order', (ev) => {
    orders$.update(orders => {
      orders.set(ev.payload.path, ev.payload);
      return orders;
    });
  });

  // Always get the current set of orders when open the window
  const ordersResponse = await invoke<Order[]>("refresh_orders");
  orders$.update(orders => {
    for (const order of ordersResponse)
      orders.set(order.path, order)
    return orders;
  });
</script>