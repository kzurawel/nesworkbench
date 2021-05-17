<script>
  import Toolbar from './components/Toolbar.svelte';
  import Statusbar from './components/Statusbar.svelte';
  import ProjectTree from './components/projecttree/ProjectTree.svelte';

  import { listen } from '@tauri-apps/api/event';
  import { onMount, onDestroy } from 'svelte';

  let unlisten;

  onMount(async () => {
    unlisten = await listen('rust-event', handleRustEvents);
  });

  onDestroy(() => {
    if (unlisten) {
      unlisten();
    }
  });

  function handleRustEvents(event) {
    console.log(event);
  }
</script>

<main>
  <Toolbar />
  <ProjectTree />
  <Statusbar />
</main>

<style>
  :global(body) {
    padding: 0;
    margin: 0;
    height: 100vh;
    max-height: 100vh;
    overflow: hidden;
  }

  :global(*) {
    box-sizing: border-box;
  }

  main {
    padding: 2em 0 1.4em 0;
    height: 100vh;
    max-height: 100vh;
    overflow: hidden;
  }
</style>
