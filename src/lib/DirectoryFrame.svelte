<script lang="ts">
  import { getMatches } from "@tauri-apps/api/cli";
  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";
  import DirectoryRoute from "./DirectoryRoute.svelte";
  import FileRow from "./FileRow.svelte";
  import FileTable from "./FileTable.svelte";
  import type { Route, FileItem } from "./model";

  let route: Route | undefined;

  let items: FileItem[] | undefined;
  listen<FileItem[]>("file-data", (event) => {
    console.log(event.payload);
    items = event.payload;
  });
  
  async function navigate(value: string | null | boolean | string[]) {
    items = undefined;
    route = await invoke<Route>('request_directory', { directory: value ? value.toString() : "" });
  }

  getMatches().then(async matches => {
    await navigate(matches.args['route'].value);
  });
</script>

{#if route}
  <DirectoryRoute {route} on:navigate={event => navigate(event.detail.route)}/>
{/if}
<div id="files">
  <FileTable>
    {#if items}
      {#each items as item (item.path)}
        <FileRow name={item.name} path={item.path} />
      {/each}
    {/if}
  </FileTable>
</div>

<style>
  #files {
    display: grid;
  }
</style>