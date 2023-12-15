// Listen to the window menu item click and decide action
TAURI.window.appWindow.onMenuClicked(({ payload: menuItemId }) => {
    switch (menuItemId) {
        case 'help':
            Windows.openHelpWindow()
            break
        case 'manual':
            Windows.openManualWindow()
            break
        case 'new_model_editor':
            Windows.openNewModelEditorWindow()
            break
        case 'local_storage':
            if (!isModelWindowAction()) break
            LiveModel.loadFromLocalStorage()
            break
        case 'aeon_import':
            if (!isModelWindowAction()) break
            UI.importAeon()
            break
        case 'sbml_import':
            if (!isModelWindowAction()) break
            UI.importSBML()
            break
        case 'g2a':
            if (!isModelWindowAction()) break
            LiveModel.handleAeonModelImport(Examples.g2a)
            break
        case 'g2b':
            if (!isModelWindowAction()) break
            LiveModel.handleAeonModelImport(Examples.g2b)
            break
        case 'budding_yeast_orlando':
            if (!isModelWindowAction()) break
            LiveModel.handleAeonModelImport(Examples.buddingYeastOrlando)
            break
        case 'budding_yeast_irons':
            if (!isModelWindowAction()) break
            LiveModel.handleAeonModelImport(Examples.buddingYeastIrons)
            break
        case 'aeon_export':
            if (!isModelWindowAction()) break
            UI.downloadAeon()
            break
        case 'sbml_export_parametrized':
            if (!isModelWindowAction()) break
            UI.downloadSBML()
            break
        case 'sbml_export_instantiated':
            if (!isModelWindowAction()) break
            UI.downloadSBMLInstantiated()
            break
        default:
            console.log("Unknown menu item id")
    }
});

// Check if menu action is supported for current window.
// If not, warn user with warning dialog.
function isModelWindowAction() {
    let currentWindowLabel = TAURI.window.getCurrent().label
    if (!currentWindowLabel.startsWith("model-window")) {
        Dialog.warningMessage(Strings.unsupportedAction)
        return false
    }
    return true
}
