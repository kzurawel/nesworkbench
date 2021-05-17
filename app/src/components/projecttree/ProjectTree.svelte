<script>
  let defaultTree = {
    name: 'Project',
    children: [
      {
        name: 'Colors',
        children: [
          {
            name: 'Palettes',
            children: []
          },
          {
            name: 'Palette Sets',
            children: []
          }
        ]
      },
      {
        name: 'Tiles',
        children: [
          {
            name: 'Tile Sets',
            children: []
          },
          {
            name: 'Pattern Tables',
            children: []
          }
        ]
      },
      {
        name: 'Screens',
        children: [
          {
            name: 'Metatiles',
            children: []
          },
          {
            name: 'Backgrounds',
            children: []
          },
          {
            name: 'Scenes',
            children: []
          }
        ]
      }
    ]
  };
  export let tree = defaultTree;
  const { name, children } = tree;
</script>

<div class="projecttree">
  <span>{name}</span>
  {#if children}
    <ul>
      {#each children as child}
        <svelte:self tree={child} />
      {/each}
    </ul>
  {/if}
</div>
