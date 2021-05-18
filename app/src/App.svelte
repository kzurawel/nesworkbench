<script>
  import Toolbar from './components/Toolbar.svelte';
  import Statusbar from './components/Statusbar.svelte';
  import Tree from './components/tree/Tree.svelte';
  import TreeNode from './components/tree/TreeNode.svelte';

  import PaletteIcon from '@svicons/boxicons-regular/Palette.svelte';
  import GridIcon from '@svicons/boxicons-regular/Grid.svelte';
  import ImageIcon from '@svicons/boxicons-regular/Image.svelte';

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
  <div class="projecttree">
    <Tree>
      <TreeNode icon parent>
        <PaletteIcon slot="icon" />
        <span slot="display">Colors</span>
        <Tree>
          <TreeNode>
            <span slot="display">Palettes</span>
            <Tree>
              <TreeNode>
                <span slot="display">Untitled Palette</span>
              </TreeNode>
            </Tree>
          </TreeNode>
          <TreeNode>
            <span slot="display">Palette Sets</span>
          </TreeNode>
        </Tree>
      </TreeNode>
      <TreeNode icon parent>
        <GridIcon slot="icon" />
        <span slot="display">Tiles</span>
        <Tree>
          <TreeNode>
            <span slot="display">Tile Sets</span>
          </TreeNode>
          <TreeNode>
            <span slot="display">Pattern Tables</span>
          </TreeNode>
        </Tree>
      </TreeNode>
      <TreeNode icon parent>
        <ImageIcon slot="icon" />
        <span slot="display">Screens</span>
        <Tree>
          <TreeNode>
            <span slot="display">Metatiles</span>
          </TreeNode>
          <TreeNode>
            <span slot="display">Backgrounds</span>
          </TreeNode>
          <TreeNode>
            <span slot="display">Scenes</span>
          </TreeNode>
        </Tree>
      </TreeNode>
    </Tree>
  </div>
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

  .projecttree {
    height: 100%;
    width: 12em;
    border-right: 1px solid #333;
    overflow: scroll;
  }

  main {
    padding: 2em 0 1.4em 0;
    height: 100vh;
    max-height: 100vh;
    overflow: hidden;
  }
</style>
