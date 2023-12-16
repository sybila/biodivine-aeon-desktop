// Listen for 'import-model' event that will pass a model from parent model editor window to this window
TAURI.event.listen("import-model", (event) => {
	const error = LiveModel.importAeon(event.payload["modelString"]);
	if (error !== undefined) {
		Dialog.errorMessage(error);
		TAURI.window.appWindow.close();
	}
});
