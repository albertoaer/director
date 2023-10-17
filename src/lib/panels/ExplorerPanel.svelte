<script>
  import DirectoryContent from "../DirectoryContent.svelte";
  import RoutingBar from "../RoutingBar.svelte";
</script>

<RoutingBar></RoutingBar>
<div id="content">
  <DirectoryContent />
</div>

<style>
  #content {
    margin-top: 0.5em;
    height: 92%;
    width: 100%;
    overflow-y: auto;
  }
</style>