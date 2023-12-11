// Listen for 'start-computation' event and start computation from (this) new window
TAURI.event.listen('start-computation', (event) => {
    const aeonString = event.payload['aeonString']
    const modelTitle = event.payload['modelTitle']
    const windowTimestamp = event.payload['windowTimestamp']

    const sessionKey = TAURI.window.getCurrent().label;
    SessionCommands.createSession(sessionKey)
    Computation.setSessionKey(sessionKey)
    Computation.setModelTitle(modelTitle)
    Computation.setWindowTimestamp(windowTimestamp)

    ComputationCommands.startComputation(aeonString, sessionKey)
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
    if (await SessionCommands.hasRunningComputation()) {
        const closeWindowWithRunningComputation = await TAURI.dialog.ask(Strings.runningComputation, {
            title: 'Running computation',
            type: 'warning',
        });
        if (!closeWindowWithRunningComputation) {
            // User decided to not close the window with running computation -> do nothing
            return
        }
    }

    SessionCommands.destroySession()
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
