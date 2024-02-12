/*
    Responsible for calling Tauri commands creating new Webview windows.
    Commands return Promises.
 */
let WindowsCommands = {

	readTextFile(path) {
		return TAURI.core.invoke("read_text_file", {
			path: path
		})
	},

	writeTextFile(path, content) {
		return TAURI.core.invoke("write_text_file", {
			path: path,
			content: content
		})
	},

	openModelWindow(windowLabel, content) {
		return TAURI.core.invoke("open_model_window", {
			label: windowLabel,
			content: content,
		});
	},

	openComputationWindow(windowLabel, windowTitle) {
		return TAURI.core.invoke("open_computation_window", {
			label: windowLabel,
			title: windowTitle
		});
	},

	openExplorerWindow(windowLabel, windowTitle) {
		return TAURI.core.invoke("open_explorer_window", {
			label: windowLabel,
			title: windowTitle
		});
	},

	openWitnessWindow(windowLabel, windowTitle) {
		return TAURI.core.invoke("open_witness_window", {
			label: windowLabel,
			title: windowTitle
		});
	},

	openTreeExplorerWindow(windowLabel, windowTitle) {
		return TAURI.core.invoke("open_tree_explorer_window", {
			label: windowLabel,
			title: windowTitle
		});
	},

	openManualWindow() {
		return TAURI.core.invoke("open_manual_window", {});
	},

	openHelpWindow() {
		return TAURI.core.invoke("open_help_window", {});
	}
};