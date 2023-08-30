let MessageDialog = {

    errorMessage(message) {
        TAURI.dialog.message(message, { title: 'Error', type: 'error' });
    }

}