<script lang="ts">
  import DirectoryBreadcrumbs from "../DirectoryBreadcrumbs.svelte";
  import DirectoryContent from "../DirectoryContent.svelte";
  import DirectoryHistory from "../DirectoryHistory.svelte";
  import { route$, childs$, navigate } from "../FSManager.svelte";

  function handleNavigate(ev: CustomEvent<{ route: string}>) {
    navigate(ev.detail.route);
  }
</script>

<DirectoryHistory />
{#if $route$}
  <DirectoryBreadcrumbs route={$route$} on:navigate={handleNavigate} />
  <div id="content">
    {#if $childs$}
      <DirectoryContent childs={$childs$} on:navigate={handleNavigate} />
    {/if}
  </div>
{/if}

<style>
  #content {
    margin-top: 0.5em;
    height: calc(100% - 4em);
    width: 100%;
    overflow-y: auto;
  }
</style>