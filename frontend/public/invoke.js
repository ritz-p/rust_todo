const invoke = window.__TAURI__.invoke

export async function fetch() {
    return await invoke("fetch", {name: name});
}