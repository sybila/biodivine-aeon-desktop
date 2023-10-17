let Dialog = {

    errorMessage(message) {
        TAURI.dialog.message(message, { title: 'Error', type: 'error' });
    },

    infoMessage(message) {
        TAURI.dialog.message(message, { title: 'Info', type: 'info' });
    },

    confirm(title, message) {
        return TAURI.dialog.ask(message, {
            title: title,
            type: 'warning',
        });
    },

}