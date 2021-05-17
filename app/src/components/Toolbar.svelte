<script>
  import NewProject from './toolbar/NewProject.svelte';
  import OpenProject from './toolbar/OpenProject.svelte';
</script>

<div>
  <NewProject />
  <OpenProject />
</div>

<style>
  div {
    border-bottom: 1px solid #111;
    display: flex;
    flex-direction: row;
    align-items: flex-start;
    height: 2em;
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
  }
</style>
