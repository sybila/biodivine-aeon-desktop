// Listen for 'send-window-session-key' event to save window session key and show the tree in window
TAURI.event.listen('send-window-session-key', (event) => {

    // Set session key for this window, so other methods can use this info
    Computation.setWindowSessionKey(event.payload['windowSessionKey'])

    console.log("got event")
    // Initialize the window
    showTree()
});

// Send message to computation window before closing this tree explorer window
TAURI.window.getCurrent().listen("tauri://close-requested", () => {
    const computationWindow = TAURI.window.WebviewWindow.getByLabel(Computation.getWindowSessionKey())
    computationWindow.emit('tree-explorer-window-closed', {})
    TAURI.window.getCurrent().close()
})
