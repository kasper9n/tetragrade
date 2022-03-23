<script lang="ts">
  import { runCmd } from './main'

  let n = 1
  async function create() {
    await runCmd('create_child_window', { id: 'child-' + n })
    n++
  }
</script>

<div>Tetragrade</div>
<button on:click={create}>Create child window {n}</button>

<style lang="sass">
  :global(body)
    font-family: Arial, Helvetica, sans-serif
    font-size: 18px
    background-color: #efe2d2
    color: #302012
    text-align: center
</style>
