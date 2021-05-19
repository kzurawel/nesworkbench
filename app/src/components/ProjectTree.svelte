<script>
  import Tree from './tree/Tree.svelte';
  import TreeNode from './tree/TreeNode.svelte';

  import PaletteIcon from '@svicons/boxicons-regular/palette.svelte';
  import GridIcon from '@svicons/boxicons-regular/grid.svelte';
  import ImageIcon from '@svicons/boxicons-regular/image.svelte';

  import { colorItems, tileItems, screenItems } from '../stores';
</script>

<div class="projecttree">
  <Tree>
    <TreeNode icon={PaletteIcon} parent>
      <svelte:fragment slot="display">Colors</svelte:fragment>
      <Tree>
        <TreeNode>
          <svelte:fragment slot="display">Palettes</svelte:fragment>
          {#if $colorItems.palettes.length}
            <Tree>
              {#each $colorItems.palettes as palette}
                <TreeNode>
                  <svelte:fragment slot="display">{palette.name}</svelte:fragment>
                </TreeNode>
              {/each}
            </Tree>
          {/if}
        </TreeNode>
        <TreeNode>
          <svelte:fragment slot="display">Palette Sets</svelte:fragment>
          {#if $colorItems.paletteSets.length}
            <Tree>
              {#each $colorItems.paletteSets as paletteSet}
                <TreeNode>
                  <svelte:fragment slot="display">{paletteSet.name}</svelte:fragment>
                </TreeNode>
              {/each}
            </Tree>
          {/if}
        </TreeNode>
      </Tree>
    </TreeNode>
    <TreeNode icon={GridIcon} parent>
      <svelte:fragment slot="display">Tiles</svelte:fragment>
      <Tree>
        <TreeNode>
          <svelte:fragment slot="display">Tile Sets</svelte:fragment>
          {#if $tileItems.tileSets.length}
            <Tree>
              {#each $tileItems.tileSets as tileSet}
                <TreeNode>
                  <svelte:fragment slot="display">{tileSet.name}</svelte:fragment>
                </TreeNode>
              {/each}
            </Tree>
          {/if}
        </TreeNode>
        <TreeNode>
          <svelte:fragment slot="display">Pattern Tables</svelte:fragment>
          {#if $tileItems.patternTables.length}
            <Tree>
              {#each $tileItems.patternTables as patternTable}
                <TreeNode>
                  <svelte:fragment slot="display">{patternTable.name}</svelte:fragment>
                </TreeNode>
              {/each}
            </Tree>
          {/if}
        </TreeNode>
      </Tree>
    </TreeNode>
    <TreeNode icon={ImageIcon} parent>
      <svelte:fragment slot="display">Screens</svelte:fragment>
      <Tree>
        <TreeNode>
          <svelte:fragment slot="display">Metatiles</svelte:fragment>
          {#if $screenItems.metatiles.length}
            <Tree>
              {#each $screenItems.metatiles as metatile}
                <TreeNode>
                  <svelte:fragment slot="display">{metatile.name}</svelte:fragment>
                </TreeNode>
              {/each}
            </Tree>
          {/if}
        </TreeNode>
        <TreeNode>
          <svelte:fragment slot="display">Backgrounds</svelte:fragment>
          {#if $screenItems.backgrounds.length}
            <Tree>
              {#each $screenItems.backgrounds as background}
                <TreeNode>
                  <svelte:fragment slot="display">{background.name}</svelte:fragment>
                </TreeNode>
              {/each}
            </Tree>
          {/if}
        </TreeNode>
        <TreeNode>
          <svelte:fragment slot="display">Scenes</svelte:fragment>
          {#if $screenItems.scenes.length}
            <Tree>
              {#each $screenItems.scenes as scene}
                <TreeNode>
                  <svelte:fragment slot="display">{scene.name}</svelte:fragment>
                </TreeNode>
              {/each}
            </Tree>
          {/if}
        </TreeNode>
      </Tree>
    </TreeNode>
  </Tree>
</div>

<style>
  .projecttree {
    height: 100%;
    width: 12em;
    border-right: 1px solid #333;
    overflow: scroll;
  }
</style>
