<script lang="ts">
  import DirectoryFrame from "./lib/DirectoryFrame.svelte";
  import TitleBar from "./lib/TitleBar.svelte";
  import Nav from "./lib/Nav.svelte";
  import { ExplorerPanel, VoidPanel } from "./lib/panels";
  import Theme from "./lib/Theme.svelte";

  import exploreIcon from '@iconify/icons-mdi/folder-outline';
  import alertsIcon from '@iconify/icons-mdi/alert-circle-outline';
  import settingsIcon from '@iconify/icons-mdi/settings-outline';
  import cleanIcon from '@iconify/icons-mdi/recycle';
</script>

<Theme />
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
        component: VoidPanel,
        icon: alertsIcon
      },
      {
        name: 'clean',
        component: VoidPanel,
        icon: cleanIcon,
      },
      {
        name: 'settings',
        component: VoidPanel,
        icon: settingsIcon
      }
    ]} />
  </DirectoryFrame>
</div>

<style>
  #app {
    height: 100vh;
    overflow: hidden;
    background-color: var(--background-color);
    color: var(--font-color);
  }
</style>