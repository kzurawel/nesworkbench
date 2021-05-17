<script>
  export let width = '2em';
  export let color = '#333';
  import FolderPlus from '@svicons/boxicons-regular/folder-plus.svelte';
</script>

<div class="icon">
  <FolderPlus {width} {color} />
</div>

<style>
  .icon {
    cursor: pointer;
  }
</style>
