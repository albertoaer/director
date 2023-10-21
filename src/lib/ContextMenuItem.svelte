<script lang="ts">
  import type { IconifyIcon } from "@iconify/svelte";
  import Icon from "@iconify/svelte";

  export let icon: string | IconifyIcon | undefined = undefined;
</script>

<tr on:click>
  <td>
    {#if icon}
      <Icon {icon} inline />
    {/if}
  </td>
  <td><slot /></td>
</tr>

<style>
  tr:hover {
    text-decoration: underline;
    cursor: pointer;
  }
</style>