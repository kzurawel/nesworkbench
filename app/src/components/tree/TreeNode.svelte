<script>
  export let icon;
  export let parent;
</script>

<div class="treenode">
  {#if icon}
    <span class="icon"><slot name="icon" /></span>
  {/if}
  <span class="display" class:parent><slot name="display" /></span>
  <slot />
</div>

<style>
  .icon {
    display: inline-block;
    width: 2em;
    height: 2em;
  }

  .display {
    vertical-align: 50%;
  }

  .display.parent {
    font-weight: bold;
  }
</style>
