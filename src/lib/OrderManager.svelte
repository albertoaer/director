<script lang="ts" context="module">
  import { writable, type Writable } from "svelte/store";
  import type { Order } from "./model/order";
  import { listen } from "@tauri-apps/api/event";

  export const orders$: Writable<Map<string, Order>> = writable(new Map());

  listen<Order>('order', (ev) => {
    orders$.update(orders => {
      orders.set(ev.payload.path, ev.payload);
      return orders;
    });
  });
</script>