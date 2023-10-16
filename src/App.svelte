<script lang="ts">
  import DirectoryFrame from "./lib/DirectoryFrame.svelte";
  import TitleBar from "./lib/TitleBar.svelte";
  import Nav from "./lib/Nav.svelte";
  import ExplorerPanel from "./lib/panels/ExplorerPanel.svelte";

  import exploreIcon from '@iconify/icons-mdi/folder-outline';
  import alertsIcon from '@iconify/icons-mdi/alert-box-outline';
  import settingsIcon from '@iconify/icons-mdi/settings-outline';
</script>

<div id="app">
  <TitleBar />
  <DirectoryFrame>
    <Nav items={[
      {
        name: 'explore',
        component: ExplorerPanel,
        icon: exploreIcon
      },
      {
        name: 'alerts',
        component: ExplorerPanel,
        icon: alertsIcon
      },
      {
        name: 'settings',
        component: ExplorerPanel,
        icon: settingsIcon
      }
    ]} />
  </DirectoryFrame>
</div>

<style>
  #app {
    height: 100vh;
    overflow: hidden;
  }
</style>