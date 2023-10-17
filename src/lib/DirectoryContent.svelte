<script lang="ts">
  import DirRow from "./DirRow.svelte";
  import DirTable from "./DirTable.svelte";
  import { DirectoryContext } from "./directory_context";

  const context = DirectoryContext.getOrSet();
  const childs$ = context.childs$;
</script>

<DirTable>
  {#if $childs$}
    {#each $childs$ as child (child.path)}
      <DirRow {child} on:navigate={event => context.navigate(event.detail.route)} />
    {/each}
  {/if}
</DirTable>