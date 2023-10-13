<script lang="ts">
  import FileRow from "./FileRow.svelte";
  import FileTable from "./FileTable.svelte";
  import { DirectoryContext } from "./directory_context";

  const context = DirectoryContext.getOrSet();
  const childs$ = context.childs$;
</script>

<FileTable>
  {#if $childs$}
    {#each $childs$ as child (child.path)}
      <FileRow name={child.name} path={child.path} on:navigate={event => context.navigate(event.detail.route)} />
    {/each}
  {/if}
</FileTable>