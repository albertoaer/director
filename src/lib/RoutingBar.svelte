<script lang="ts">
  import DirectoryBreadcrumbs from "./DirectoryBreadcrumbs.svelte";
  import { DirectoryContext } from "./directory_context";

  const context = DirectoryContext.getOrSet();
  const route$ = context.route$;
  
</script>
<DirectoryBreadcrumbs route={$route$} on:navigate={event => context.navigate(event.detail.route)}/>