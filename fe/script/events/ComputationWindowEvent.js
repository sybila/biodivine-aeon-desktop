// Listen for 'start-computation' event and start computation from (this) new window
TAURI.event.listen('start-computation', (event) => {
    const aeonString = event.payload['aeonString']
    const modelTitle = event.payload['modelTitle']
    const windowTimestamp = event.payload['windowTimestamp']

    const windowSessionKey = TAURI.window.getCurrent().label;
    Session.createWindowSession(windowSessionKey)
    Computation.setWindowSessionKey(windowSessionKey)
    Computation.setModelTitle(modelTitle)
    Computation.setWindowTimestamp(windowTimestamp)

    ComputationEndpoints.startComputation(aeonString, windowSessionKey)
        .then((startTimestamp) => {
            console.log("Started computation ", startTimestamp)
            Computation.setLastComputation(startTimestamp)
            Computation.update_computation_process()
        })
        .catch((errorMessage) => {
            Dialog.errorMessage(errorMessage)
        });
});


// Destroy computation session when window is closed
TAURI.window.getCurrent().listen("tauri://close-requested", async () => {

    // If computation is still running, ask user to confirm
    if (await Session.hasRunningComputation()) {
        const closeWindowWithRunningComputation = await TAURI.dialog.ask(Strings.runningComputation, {
            title: 'Running computation',
            type: 'warning',
        });
        if (!closeWindowWithRunningComputation) {
            // User decided to not close the window with running computation -> do nothing
            return
        }
    }

    Session.destroyWindowSession()
        .then(async (okMessage) => {
            const currentWindow = TAURI.window.getCurrent();
            await currentWindow.close();
        })
        .catch((errorMessage) => {
            Dialog.errorMessage(errorMessage)
        })
})

// Listen for event when tree explorer window is closed
TAURI.event.listen('tree-explorer-window-closed', () => {
    Computation.setTreeExplorerWindowLabel(undefined)
});
