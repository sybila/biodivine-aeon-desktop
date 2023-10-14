let MessageDialog = {

    errorMessage(message) {
        TAURI.dialog.message(message, { title: 'Error', type: 'error' });
    },

    infoMessage(message) {
        TAURI.dialog.message(message, { title: 'Info', type: 'info' });
    }

}