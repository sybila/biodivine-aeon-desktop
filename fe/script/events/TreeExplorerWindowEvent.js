// Listen for 'send-window-session-key' event to save window session key and show the tree in window
TAURI.event.listen('send-window-session-key', (event) => {

    // Set session key for this window, so other methods can use this info
    Computation.setSessionKey(event.payload['sessionKey'])

    // Initialize the window
    showTree()
});

// Send message to computation window before closing this tree explorer window
TAURI.window.getCurrent().listen("tauri://close-requested", async () => {
    let confirmClose = true
    if (UI.isWaitingForResult()) {
        confirmClose = await Dialog.confirm("Waiting for result", Strings.waitingForTreeResult)
    }

    if (confirmClose) {
        const computationWindow = TAURI.window.WebviewWindow.getByLabel(Computation.getSessionKey())
        computationWindow.emit('tree-explorer-window-closed', {})
        TAURI.window.getCurrent().close()
    }
})
