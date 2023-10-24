// Listen to the window menu item click and decide action
TAURI.window.appWindow.onMenuClicked(({ payload: menuItemId }) => {
    switch (menuItemId) {
        case 'help':
            UI.openHelpWindow()
            break
        case 'manual':
            WindowsEndpoints.openManualWindow()
            break
        case 'local_storage':
            LiveModel.loadFromLocalStorage()
            break
        case 'aeon_import':
            UI.importAeon()
            break
        case 'sbml_import':
            UI.importSBML()
            break
        case 'g2a':
            LiveModel.handleAeonModelImport(Examples.g2a)
            break
        case 'g2b':
            LiveModel.handleAeonModelImport(Examples.g2b)
            break
        case 'budding_yeast_orlando':
            LiveModel.handleAeonModelImport(Examples.buddingYeastOrlando)
            break
        case 'budding_yeast_irons':
            LiveModel.handleAeonModelImport(Examples.buddingYeastIrons)
            break
        case 'aeon_export':
            UI.downloadAeon()
            break
        case 'sbml_export_parametrized':
            UI.downloadSBML()
            break
        case 'sbml_export_instantiated':
            UI.downloadSBMLInstantiated()
            break
        default:
            console.log("Unknown menu item id")
    }
});
