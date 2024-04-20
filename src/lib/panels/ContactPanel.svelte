<script>
  import GithubFace from "@iconify/icons-mdi/github-face";
  import GithubCircle from "@iconify/icons-mdi/github-circle";
  import Icon from "@iconify/svelte";
</script>

<h1>
  Developer Info
</h1>

<a href="https://github.com/albertoaer" target="_blank"><Icon icon={GithubFace} /> Alberto Elorza</a>
<br />
<a href="https://github.com/albertoaer/director" target="_blank"><Icon icon={GithubCircle} /> Repository</a>

<style>
  a {
    color: var(--font-color);
  }

  a:hover {
    text-decoration: dotted underline;
  }
</style>