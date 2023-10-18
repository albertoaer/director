<script lang="ts">
  import Icon, { type IconifyIcon } from "@iconify/svelte";

  export let icon: string | IconifyIcon;
</script>

<button>
  <Icon {icon} />
</button>

<style>
  button {
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: var(--item-color);
    color: inherit;
    font-size: calc(0.5em + 8vh);
    width: 1em;
    height: 1em;
    border: none;
    margin: 0;
    border-radius: 100%;
    box-shadow: 0px 5px 10px 0px black;
    position: absolute;
    bottom: 1em;
    right: 1em;
    transition: 400ms all ease;
  }
  
  button:hover {
    background-color: var(--item-active-color);
    box-shadow: 0px 5px 25px 0px black;
    cursor: pointer;
    transition: 300ms all ease-out;
  }
</style>