<script>
  export let icon = false;
  export let parent = false;
  $: hasIcon = icon != false;
</script>

<div class="treenode">
  {#if hasIcon}
    <span class="icon"><svelte:component this={icon} /></span>
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
