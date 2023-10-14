// Listen for 'get-attractors' event to show attractors in explorer window
TAURI.event.listen('get-attractors', (event) => {

    // Set session key for this window, so other methods can use this info
    Computation.setWindowSessionKey(event.payload['windowSessionKey'])

    const behavior = event.payload['behavior']
    ComputationResultsEndpoints.getAttractors(behavior)
        .then((okJson) => {
            RESULT = okJson;

            if(okJson['has_large_attractors']) {
                MessageDialog.infoMessage("Some attractors were too large to draw. These will be shown only as two states with the constant and non-constant variables differentiated.");
            }

            for (let i = 0; i < okJson['attractors'].length; i++) {
                okJson['attractors'][i].vis = edgesToVisFormat(okJson['attractors'][i].graph);
            }

            addLabels();
            displayAll();
            network.on('click', nodeClick);
            document.getElementById('explorer-update-functions').innerHTML = generateWitness()
        })
        .catch((errorMessage) => {
            MessageDialog.errorMessage(errorMessage)
        })
});