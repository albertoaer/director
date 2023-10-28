<script lang="ts">
  import TitleBar from "./lib/TitleBar.svelte";
  import Nav from "./lib/Nav.svelte";
  import { ExplorerPanel, VoidPanel, AlertsPanel, OrdersPanel } from "./lib/panels";
  import Theme from "./lib/Theme.svelte";
  import DirectoryHistory from "./lib/DirectoryHistory.svelte";

  import ExploreIcon from '@iconify/icons-mdi/folder-outline';
  import AlertsIcon from '@iconify/icons-mdi/alert-circle-outline';
  import CleanIcon from '@iconify/icons-mdi/recycle';
  import SettingsIcon from '@iconify/icons-mdi/settings-outline';
  import OrderIcon from "@iconify/icons-mdi/state-machine";
</script>

<Theme />
<DirectoryHistory />

<div id="app">
  <TitleBar />
  <Nav items={[
    {
      name: 'explore',
      component: ExplorerPanel,
      icon: ExploreIcon
    },
    {
      name: 'alerts',
      component: AlertsPanel,
      icon: AlertsIcon
    },
    {
      name: 'orders',
      component: OrdersPanel,
      icon: OrderIcon
    },
    {
      name: 'settings',
      component: VoidPanel,
      icon: SettingsIcon
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