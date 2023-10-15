// Listen for 'get-attractors' event to show attractors in explorer window
TAURI.event.listen('get-attractors', (event) => {
    Computation.setWindowSessionKey(event.payload['windowSessionKey'])

    const behavior = event.payload['behavior']
    ComputationResultsEndpoints.getAttractors(behavior)
        .then((okJson) => {
            initExplorer(okJson)
        })
        .catch((errorMessage) => {
            MessageDialog.errorMessage(errorMessage)
        })
});

// Listen for 'get-tree-attractors' event to show attractors in explorer window
TAURI.event.listen('get-tree-attractors', (event) => {
    Computation.setWindowSessionKey(event.payload['windowSessionKey'])

    const node = event.payload['node']
    ComputationResultsEndpoints.getTreeAttractors(node)
        .then((okJson) => {
            initExplorer(okJson)
        })
        .catch((errorMessage) => {
            MessageDialog.errorMessage(errorMessage)
        })
});

// Listen for 'get-stability-attractors' event to show attractors in explorer window
TAURI.event.listen('get-stability-attractors', (event) => {
    Computation.setWindowSessionKey(event.payload['windowSessionKey'])

    const node = event.payload['node']
    const behavior = event.payload['behavior']
    const variable = event.payload['variable']
    const vector = event.payload['vector']
    ComputationResultsEndpoints.getStabilityAttractors(node, behavior, variable, vector)
        .then((okJson) => {
            initExplorer(okJson)
        })
        .catch((errorMessage) => {
            MessageDialog.errorMessage(errorMessage)
        })
});
