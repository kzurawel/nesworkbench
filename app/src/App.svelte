<script>
  import Toolbar from './components/Toolbar.svelte';
  import Statusbar from './components/Statusbar.svelte';
</script>

<main>
  <Toolbar/>
  <Statusbar/>
</main>

<style>
  :global(body) {
    padding: 0;
    margin: 0;
    height: 100vh;
    max-height: 100vh;
    overflow: hidden;
  }
</style>
