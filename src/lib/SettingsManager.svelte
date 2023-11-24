<script lang="ts" context="module">
  import { message } from "@tauri-apps/api/dialog";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/tauri";
  import { writable, type Writable } from "svelte/store";

  export const startup$: Writable<string[]> = writable([]);

  listen<string[]>("startup-update", event => startup$.set(event.payload));

  export function add_startup(directory: string) {
    return invoke("add_startup", { directory });
  }

  export function remove_startup(directory: string) {
    return invoke("remove_startup", { directory });
  }

  invoke<string[]>('get_startup').then(startup$.set);

  invoke('run_startup').catch(err => message(err, {
    type: 'error'
  }));
</script>