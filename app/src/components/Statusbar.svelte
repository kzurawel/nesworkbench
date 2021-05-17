<script>
  export let status = 'Welcome to NES Workbench';
</script>

<div class="statusbar">
  <p>{status}</p>
</div>

<style>
  .statusbar {
    border-top: 1px solid #111;
    height: 1.4em;
    padding: 0 4px;
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
  }

  p {
    margin: 0;
    padding: 0;
  }
</style>
