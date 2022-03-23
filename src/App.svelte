<script lang="ts">
  import { invoke } from '@tauri-apps/api'

  export function popup(msg: string) {
    invoke('error_popup', { msg })
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  export async function runCmd<T = any>(cmd: string, options: { [key: string]: any } = {}) {
    return (await invoke(cmd, options).catch((msg) => {
      popup(msg)
      throw msg
    })) as T
  }

  async function create() {
    await runCmd('create_child_window')
  }
  create()
  function click(e: Event) {
    console.log(e.type, e)
  }
</script>

<svelte:window on:mouseover={click} on:mousedown={click} on:mouseup={click} on:focus={click} />

<div>Tetragrade</div>

<style lang="sass">
  :global(body)
    font-family: Arial, Helvetica, sans-serif
    font-size: 18px
    background-color: #efe2d2
    color: #302012
    text-align: center
</style>
