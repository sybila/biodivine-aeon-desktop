function init() {
    // Emit when the window is fully initialized and ready
    TAURI.event.emit('ready', {});
}