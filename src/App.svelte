<script lang="ts">
  import DirectoryFrame from "./lib/DirectoryFrame.svelte";
  import TitleBar from "./lib/TitleBar.svelte";
  import RoutingBar from "./lib/RoutingBar.svelte";
  import DirectoryContent from "./lib/DirectoryContent.svelte";
</script>

<div id="app">
  <TitleBar />
  <DirectoryFrame>
    <RoutingBar />
    <div id="content">
      <DirectoryContent />
    </div>
  </DirectoryFrame>
</div>

<style>
  #app {
    height: 100vh;
    overflow: hidden;
  }

  #content {
    height: 92%;
    width: 100%;
    overflow-y: auto;
  }
</style>