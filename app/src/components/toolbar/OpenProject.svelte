<script>
  export let width = '2em';
  export let color = '#333';
  import FolderOpen from '@svicons/boxicons-regular/folder-open.svelte';
</script>

<div class='icon'>
  <FolderOpen {width} {color}/>
</div>

<style>
  .icon {
    cursor: pointer;
  }
</style>
