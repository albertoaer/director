<script lang="ts" context="module">
  import { writable, type Writable } from "svelte/store";
  import type { FSChild, FSEvent, Route } from "./model/fs";
  import { getMatches } from "@tauri-apps/api/cli";
  import { invoke } from "@tauri-apps/api/tauri";
  import { message } from "@tauri-apps/api/dialog";
  import { listen } from "@tauri-apps/api/event";

  let route: Route | undefined;
  export const route$: Writable<Route | undefined> = writable();
  export const childs$: Writable<FSChild[] | undefined> = writable();

  listen<FSEvent>("updated-entry", (event) => {
    if (event.payload.entry?.path == route?.path && route?.path) {
      childs$.set(event.payload.entry?.childs);
    }
  });

  export async function navigate(directory: string, historyRecord: boolean = true) {
    try {
      route = await invoke<Route>('request_calculate_directory', { directory });
      route$.set(route);
    } catch (err: any) {
      message(err.toString(), { type: 'error' });
    }
    // at this point the route is ensured
    if (historyRecord)
      history.pushState({route}, '', '');
  }

  getMatches().then(async matches => {
    const value = matches.args['route'].value;
    await navigate(value ? value.toString() : "");
  });
</script>