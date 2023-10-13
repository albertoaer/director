<script lang="ts">
  import { getMatches } from "@tauri-apps/api/cli";
  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";
  import DirectoryBreadcrumbs from "./DirectoryBreadcrumbs.svelte";
  import FileRow from "./FileRow.svelte";
  import FileTable from "./FileTable.svelte";
  import type { FSChild, FSEvent, Route } from "./model";

  let route: Route | undefined;

  let items: FSChild[] | undefined;
  listen<FSEvent>("updated-entry", (event) => {
    if (event.payload.entry?.path == route?.path && route?.path) {
      items = event.payload.entry?.data;
    }
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
  <DirectoryBreadcrumbs {route} on:navigate={event => navigate(event.detail.route)}/>
{/if}
<div id="files">
  <FileTable>
    {#if items}
      {#each items as item (item.path)}
        <FileRow name={item.name} path={item.path} on:navigate={event => navigate(event.detail.route)} />
      {/each}
    {/if}
  </FileTable>
</div>

<style>
  #files {
    display: grid;
  }
</style>