let ComputationResultsEndpoints = {

    openWitnessInNewWindow(witness) {
        TAURI.invoke('get_witness', {
            classStr: witness,
            windowSessionKey: Computation._windowSessionKey
        })
            .then(async (witness) => {
                const witnessWindowLabel = await LiveModel.openModelInNewWindow(witness)
                if (witnessWindowLabel !== null) {
                    const witnessWindow = TAURI.window.WebviewWindow.getByLabel(witnessWindowLabel)

                    // Wait until the new window is initialized
                    witnessWindow.once('ready', () => {
                        // Emit to open model editor tab in new window
                        witnessWindow.emit('open-editor-tab', {})
                    })
                }
            })
            .catch((errorMessage) => {
                MessageDialog.errorMessage(errorMessage)
            })
    },

    openAttractorExplorerInNewWindow(behaviour) {

    }
}