<script lang="ts">
  export let grow = false;
</script>

<button on:click class:grow ><slot /></button>

<style>
  button {
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: var(--item-color);
    color: inherit;
    font-size: 1em;
    padding: 0.6em;
    border: none;
    margin: 0;
    border-radius: 5px;
    box-shadow: 0px 5px 10px 0px black;
    transition: 400ms all ease;
    user-select: none;
  }

  button.grow {
    flex-grow: 1;
  }
  
  button:hover {
    background-color: var(--item-active-color);
    box-shadow: 0px 10px 10px 0px black;
    cursor: pointer;
    transition: 300ms all ease-out;
  }

  button:active {
    background-color: var(--item-active-color);
    box-shadow: 0px 5px 5px 0px black;
    cursor: pointer;
    transition: 200ms all ease-out;
  }
</style>