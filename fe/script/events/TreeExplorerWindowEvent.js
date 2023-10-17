// Listen for 'send-window-session-key' event to save window session key and show the tree in window
TAURI.event.listen('send-window-session-key', (event) => {

    // Set session key for this window, so other methods can use this info
    Computation.setWindowSessionKey(event.payload['windowSessionKey'])

    console.log("got event")
    // Initialize the window
    showTree()
});