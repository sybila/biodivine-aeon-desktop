// Listen for 'import-model' event that will pass model from main window to this window
TAURI.event.listen('import-model', (event) => {
    const error = LiveModel.importAeon(event.payload['modelString'])
    if (error !== undefined) {
        MessageDialog.errorMessage(error)
        TAURI.window.appWindow.close();
    }
});