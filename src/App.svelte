<script lang="ts">
  import TitleBar from "./lib/TitleBar.svelte";
  import Nav from "./lib/Nav.svelte";
  import { ExplorerPanel, VoidPanel, AlertsPanel } from "./lib/panels";
  import Theme from "./lib/Theme.svelte";

  import exploreIcon from '@iconify/icons-mdi/folder-outline';
  import alertsIcon from '@iconify/icons-mdi/alert-circle-outline';
  import cleanIcon from '@iconify/icons-mdi/recycle';
  import settingsIcon from '@iconify/icons-mdi/settings-outline';
</script>

<Theme />
<div id="app">
  <TitleBar />
  <Nav items={[
    {
      name: 'explore',
      component: ExplorerPanel,
      icon: exploreIcon
    },
    {
      name: 'alerts',
      component: AlertsPanel,
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
</div>

<style>
  #app {
    height: 100vh;
    overflow: hidden;
    background-color: var(--background-color);
    color: var(--font-color);
  }
</style>