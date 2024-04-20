<script lang="ts">
  import TitleBar from "./lib/TitleBar.svelte";
  import Nav from "./lib/Nav.svelte";
  import { ExplorerPanel, AlertsPanel, OrdersPanel, ContactPanel } from "./lib/panels";
  import Theme from "./lib/Theme.svelte";
  import DirectoryHistory from "./lib/DirectoryHistory.svelte";

  import ExploreIcon from '@iconify/icons-mdi/folder-outline';
  import AlertsIcon from '@iconify/icons-mdi/alert-circle-outline';
  import SettingsIcon from '@iconify/icons-mdi/settings-outline';
  import ContactIcon from '@iconify/icons-mdi/contact';
  import OrderIcon from "@iconify/icons-mdi/state-machine";
  import SettingsPanel from "./lib/panels/SettingsPanel.svelte";
</script>

<svelte:window on:contextmenu={ev => ev.preventDefault()}/>

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
      component: SettingsPanel,
      icon: SettingsIcon
    },
    {
      name: 'contact',
      component: ContactPanel,
      icon: ContactIcon,
      separator: true
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