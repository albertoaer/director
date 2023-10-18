<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import DirRow from "./DirRow.svelte";
  import DirTable from "./DirTable.svelte";
  import type { FSChild } from "./model";

  const dispatch = createEventDispatcher<{ navigate: { route: string } }>();

  export let childs: FSChild[];
</script>

<DirTable let:sizeUnit>
  {#each childs as child (child.path)}
    <DirRow
      {child}
      mapSize={size => size * sizeUnit.factor + sizeUnit.symbol}
      on:navigate={event => dispatch("navigate", { route: event.detail.route })}
    />
  {/each}
</DirTable>