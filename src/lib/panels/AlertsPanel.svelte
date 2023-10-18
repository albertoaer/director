<script lang="ts">
  import DirectoryContent from "../DirectoryContent.svelte";
  import type { FSChild } from "../model";

  const childs: FSChild[] = []
</script>

<h2>These files have set off the alarms</h2>

<DirectoryContent {childs} />

<style>
  h2 {
    margin: 0;
    margin-bottom: 1em;
    text-align: center;
  }
</style>