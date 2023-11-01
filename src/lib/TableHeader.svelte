<script lang="ts">
  export let action: boolean = false;
  export let width: "auto" | `${number}${'%' | 'rm' | 'px'}` = "auto";
</script>

<th on:click class:action style="width: {width};"><slot /></th>

<style>
  th {
    padding: 0.3em 0.5em;
    user-select: none;
  }
  
  th.action:hover {
    text-decoration: underline;
    cursor: pointer;
  }
</style>