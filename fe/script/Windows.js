let Windows = {

    // Open model in new window and return label of this window
    async openModelInNewWindow(modelString) {
        let windowLabel = 'model-window:' + Date.now()

        await TAURI.invoke("open_model_window", {
            label: windowLabel
        })
            .then(() => {
                const newModelWindow = TAURI.window.WebviewWindow.getByLabel(windowLabel)

                // Wait until the new window is initialized
                newModelWindow.once('ready', () => {
                    // Emit model string to the new window
                    newModelWindow.emit('import-model', { modelString: modelString })
                })
            }).catch((errorMessage) => {
                MessageDialog.errorMessage(errorMessage)
                windowLabel = null
            })

        return windowLabel
    },

    openComputationWindow(aeonString) {
        let windowLabel = 'computation-window:' + Date.now()

        TAURI.invoke("open_computation_window", {
            label: windowLabel
        })
            .then(() => {
                const newComputationWindow = TAURI.window.WebviewWindow.getByLabel(windowLabel)

                // Wait until the new window is initialized
                newComputationWindow.once('ready', () => {
                    // Emit to the new computation window to start computation
                    newComputationWindow.emit('start-computation', { aeonString: aeonString })
                })
            }).catch((errorMessage) => {
            MessageDialog.errorMessage(errorMessage)
        })
    },

    openWitnessWindow(witness) {
        ComputationResultsEndpoints.getWitness(witness)
            .then(async (witness) => {
                const witnessWindowLabel = await this.openModelInNewWindow(witness)
                if (witnessWindowLabel !== null) {
                    const witnessWindow = TAURI.window.WebviewWindow.getByLabel(witnessWindowLabel)

                    // Wait until the new window is initialized
                    witnessWindow.once('ready', () => {
                        // Emit to open model editor tab in new witness window
                        witnessWindow.emit('open-editor-tab', {})
                    })
                }
            })
            .catch((errorMessage) => {
                MessageDialog.errorMessage(errorMessage)
            })
    },

    openExplorerWindow(behavior) {
        let explorerWindowLabel = "explorer-window:" + Date.now()

        TAURI.invoke('open_explorer_window', {
            label: explorerWindowLabel
        })
            .then(() => {
                const newExplorerWindow = TAURI.window.WebviewWindow.getByLabel(explorerWindowLabel)

                // Wait until the new explorer window is initialized
                newExplorerWindow.once('ready', () => {
                    // Emit to get attractors
                    newExplorerWindow.emit('get-attractors', { behavior: behavior, windowSessionKey: Computation.getWindowSessionKey()})
                })
            }).catch((errorMessage) => {
                MessageDialog.errorMessage(errorMessage)
            })
    },

}