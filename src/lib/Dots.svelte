<script lang="ts">
  export let cond: boolean;
</script>

<span class:cond></span>

<style>
  @keyframes dots {
    0%   { content: '.  ';   }
    50%  { content: '.. ';  }
    100% { content: '...'; }
  }

  span.cond::after {
    content: '';
    animation: dots 1s infinite linear;
    animation-delay: 0;
  }
</style>