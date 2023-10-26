// Listen for 'get-attractors' event to show attractors in explorer window
TAURI.event.listen('get-attractors', (event) => {
    Computation.setWindowSessionKey(event.payload['windowSessionKey'])

    const behavior = event.payload['behavior']

    UI.isLoading(true)
    ComputationResultsEndpoints.getAttractors(behavior)
        .then((okJson) => {
            UI.isLoading(false)
            showInExplorer(okJson)
        })
        .catch((errorMessage) => {
            UI.isLoading(false)
            Dialog.errorMessage(errorMessage)
        })
});

// Listen for 'get-tree-attractors' event to show attractors in explorer window
TAURI.event.listen('get-tree-attractors', (event) => {
    Computation.setWindowSessionKey(event.payload['windowSessionKey'])

    const node = event.payload['node']

    UI.isLoading(true)
    ComputationResultsEndpoints.getTreeAttractors(node)
        .then((okJson) => {
            UI.isLoading(false)
            showInExplorer(okJson)
        })
        .catch((errorMessage) => {
            UI.isLoading(false)
            Dialog.errorMessage(errorMessage)
        })
});

// Listen for 'get-stability-attractors' event to show attractors in explorer window
TAURI.event.listen('get-stability-attractors', (event) => {
    Computation.setWindowSessionKey(event.payload['windowSessionKey'])

    const node = event.payload['node']
    const behavior = event.payload['behavior']
    const variable = event.payload['variable']
    const vector = event.payload['vector']

    UI.isLoading(true)
    ComputationResultsEndpoints.getStabilityAttractors(node, behavior, variable, vector)
        .then((okJson) => {
            UI.isLoading(false)
            showInExplorer(okJson)
        })
        .catch((errorMessage) => {
            UI.isLoading(false)
            Dialog.errorMessage(errorMessage)
        })
});
