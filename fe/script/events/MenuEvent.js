TAURI.window.getCurrent().listen("tauri://focus",  () => {
    let label = TAURI.window.getCurrent().label
    console.log(label)
    TAURI.invoke('update_menu_items', { label: label })
})

let label = TAURI.window.getCurrent().label
console.log(label)
TAURI.invoke('update_menu_items', { label: label })