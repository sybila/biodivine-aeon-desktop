let Dialog = {

	infoMessage(message) {
		TAURI.dialog.message(message, { title: "Info", type: "info" });
	},

	warningMessage(message) {
		TAURI.dialog.message(message, { title: "Warning", type: "warning" });
	},

	errorMessage(message) {
		TAURI.dialog.message(message, { title: "Error", type: "error" });
	},

	confirm(title, message) {
		return TAURI.dialog.ask(message, {
			title: title,
			type: "warning",
		});
	},

};