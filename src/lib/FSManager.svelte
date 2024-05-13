<script lang="ts" context="module">
  import { writable, type Writable } from "svelte/store";
  import type { FSChild, FSEntriesUpdate, Route } from "./model/fs";
  import { getMatches } from "@tauri-apps/api/cli";
  import { invoke } from "@tauri-apps/api/tauri";
  import { message } from "@tauri-apps/api/dialog";
  import { listen } from "@tauri-apps/api/event";

  let route: Route | undefined;
  export const route$: Writable<Route | undefined> = writable();
  export const childs$: Writable<FSChild[] | undefined> = writable();

  listen<FSEntriesUpdate>("updated-entries", event => {
    if (event.payload.path == route?.path && route?.path) {
      childs$.set(event.payload.childs);
    }
  });

  export interface NavigateOptions {
    historyRecord: boolean,
    calculate: boolean
  }

  export async function navigate(directory: string, options: Partial<NavigateOptions> | undefined = undefined) {
    try {
      const cmd = options?.calculate ? 'request_calculate_directory' : 'request_directory';
      route = await invoke<Route>(cmd, { directory });
      route$.set(route);
    } catch (err: any) {
      message(err.toString(), { type: 'error' });
    }
    // at this point the route is ensured
    if (options?.historyRecord ?? true)
      history.pushState({route}, '', '');
  }

  getMatches().then(async matches => {
    const value = matches.args['route'].value;
    await navigate(value ? value.toString() : "");
  });

  listen<string>("update-cwd", event => navigate(event.payload));
</script>