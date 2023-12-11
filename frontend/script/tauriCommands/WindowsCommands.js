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

    openExplorerWindow(windowLabel) {
        return TAURI.invoke('open_explorer_window', {
            label: windowLabel,
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