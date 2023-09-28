// Listens for 'import-model' event that will pass model from main window to this window
TAURI.event.listen('import-model', (event) => {
    const error = LiveModel.importAeon(event.payload['modelString'])
    if (error !== undefined) {
        MessageDialog.errorMessage(error)
        TAURI.window.appWindow.close();
    }
});

// Destroys window computation session when window is closed
TAURI.window.getCurrent().listen("tauri://close-requested", () => {
    const currentWindow = TAURI.window.getCurrent();
    TAURI.invoke('remove_window_session', { windowLabel: currentWindow.label })
    currentWindow.close();
})