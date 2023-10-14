// Listen for 'start-computation' event and start computation from (this) new window
TAURI.event.listen('start-computation', (event) => {
    const aeonString = event.payload['aeonString']

    const windowSessionKey = TAURI.window.getCurrent().label;
    Session.createWindowSession(windowSessionKey)

    ComputationEndpoints.startComputation(aeonString, windowSessionKey)
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
            MessageDialog.errorMessage(errorMessage)
        })
})
