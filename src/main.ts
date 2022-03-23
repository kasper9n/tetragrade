import App from './App.svelte'
import { invoke } from '@tauri-apps/api'

const app = new App({
  target: document.body,
})

export default app

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
