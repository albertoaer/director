<script lang="ts">
  import DirectoryBreadcrumbs from "../DirectoryBreadcrumbs.svelte";
  import DirectoryContent from "../DirectoryContent.svelte";
  import { DirectoryContext } from "../directory_context";

  const context = DirectoryContext.getOrSet();
  const route$ = context.route$;
  const childs$ = context.childs$;

  function navigate(ev: CustomEvent<{ route: string}>) {
    context.navigate(ev.detail.route);
  }
</script>

<DirectoryBreadcrumbs route={$route$} on:navigate={navigate} />
<div id="content">
  {#if $childs$}
    <DirectoryContent childs={$childs$} on:navigate={navigate} />
  {/if}
</div>

<style>
  #content {
    margin-top: 0.5em;
    height: calc(100% - 4em);
    width: 100%;
    overflow-y: auto;
  }
</style>