/*
    Responsible for calling Tauri commands creating new Webview windows.
    Commands return Promises.
 */
let WindowsCommands = {

    openModelWindow(windowLabel) {
        return TAURI.invoke("open_model_window", {
            label: windowLabel
        })
    },

    openComputationWindow(windowLabel, windowTitle) {
        return TAURI.invoke("open_computation_window", {
            label: windowLabel,
            title: windowTitle
        })
    },

    openExplorerWindow(windowLabel, windowTitle) {
        return TAURI.invoke('open_explorer_window', {
            label: windowLabel,
            title: windowTitle
        })
    },

    openWitnessWindow(windowLabel, windowTitle) {
        return TAURI.invoke('open_witness_window', {
            label: windowLabel,
            title: windowTitle
        })
    },

    openTreeExplorerWindow(windowLabel, windowTitle) {
        return TAURI.invoke('open_tree_explorer_window', {
            label: windowLabel,
            title: windowTitle
        })
    },

    openManualWindow() {
        return TAURI.invoke('open_manual_window', {})
    },

    openHelpWindow() {
        return TAURI.invoke('open_help_window', {})
    }
}