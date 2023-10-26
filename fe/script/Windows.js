let Windows = {

    // Open model in new window and return label of this window
    async openModelInNewWindow(modelString) {
        let windowLabel = 'model-window:' + Date.now()

        await WindowsEndpoints.openModelWindow(windowLabel)
            .then(() => {
                const newModelWindow = TAURI.window.WebviewWindow.getByLabel(windowLabel)

                // Wait until the new window is initialized
                newModelWindow.once('ready', () => {
                    // Emit model string to the new window
                    newModelWindow.emit('import-model', { modelString: modelString })
                })
            }).catch((errorMessage) => {
                Dialog.errorMessage(errorMessage)
                windowLabel = null
            })

        return windowLabel
    },

    openComputationWindow(aeonString) {
        let timestamp = Date.now()
        let windowLabel = 'computation-window:' + timestamp

        let modelTitle = Model.getModelName(aeonString)
        if (modelTitle === undefined || modelTitle.length < 1) {
            let filePath = ModelEditor.getModelFilePath()
            modelTitle = filePath !== undefined ? filePath : "Model without name"
        }

        let windowTitle = modelTitle + ", started: " + new Date(timestamp).toLocaleTimeString('en-GB')

        WindowsEndpoints.openComputationWindow(windowLabel, windowTitle)
            .then(() => {
                const newComputationWindow = TAURI.window.WebviewWindow.getByLabel(windowLabel)

                // Wait until the new window is initialized
                newComputationWindow.once('ready', () => {
                    // Emit to the new computation window to start computation
                    newComputationWindow.emit('start-computation', { aeonString: aeonString, modelTitle: modelTitle, windowTimestamp: timestamp })
                })
            }).catch((errorMessage) => {
                Dialog.errorMessage(errorMessage)
        })
    },

    async newWitnessWindow(witness) {
        const witnessWindowLabel = await this.openModelInNewWindow(witness, true)
        if (witnessWindowLabel !== null) {
            const witnessWindow = TAURI.window.WebviewWindow.getByLabel(witnessWindowLabel)

            // Emit to open model editor tab in new witness window
            witnessWindow.emit('open-editor-tab', {})
        }
    },

    openWitnessWindow(witness) {
        UI.isLoading(true)
        ComputationResultsEndpoints.getWitness(witness)
            .then((witness) => {
                UI.isLoading(false)
                this.newWitnessWindow(witness)
            })
            .catch((errorMessage) => {
                UI.isLoading(false)
                Dialog.errorMessage(errorMessage)
            })
    },

    openTreeWitnessWindow(node) {
        UI.isLoading(true)
        ComputationResultsEndpoints.getTreeWitness(node)
            .then((witness) => {
                UI.isLoading(false)
                this.newWitnessWindow(witness)
            })
            .catch((errorMessage) => {
                UI.isLoading(false)
                Dialog.errorMessage(errorMessage)
            })
    },

    openStabilityWitnessWindow(node, behavior, variable, vector) {
        UI.isLoading(true)
        ComputationResultsEndpoints.getStabilityWitness(node, behavior, variable, vector)
            .then((witness) => {
                UI.isLoading(false)
                this.newWitnessWindow(witness)
            })
            .catch((errorMessage) => {
                UI.isLoading(false)
                Dialog.errorMessage(errorMessage)
            })
    },

    openAttractorExplorerWindow(behavior) {
        let explorerWindowLabel = "explorer-window:" + Date.now()

        WindowsEndpoints.openExplorerWindow(explorerWindowLabel)
            .then(() => {
                const newExplorerWindow = TAURI.window.WebviewWindow.getByLabel(explorerWindowLabel)

                // Wait until the new explorer window is initialized
                newExplorerWindow.once('ready', () => {
                    // Emit to get attractors
                    newExplorerWindow.emit('get-attractors', {
                        behavior: behavior,
                        windowSessionKey: Computation.getWindowSessionKey()
                    })
                })
            }).catch((errorMessage) => {
                Dialog.errorMessage(errorMessage)
            })
    },

    openTreeAttractorExplorerWindow(node) {
        let explorerWindowLabel = "explorer-window:" + Date.now()

        WindowsEndpoints.openExplorerWindow(explorerWindowLabel)
            .then(() => {
                const newExplorerWindow = TAURI.window.WebviewWindow.getByLabel(explorerWindowLabel)

                // Wait until the new explorer window is initialized
                newExplorerWindow.once('ready', () => {
                    // Emit to get tree attractors
                    newExplorerWindow.emit('get-tree-attractors', {
                        node: node,
                        windowSessionKey: Computation.getWindowSessionKey()
                    })
                })
            }).catch((errorMessage) => {
            Dialog.errorMessage(errorMessage)
        })
    },

    openStabilityAttractorExplorerWindow(node, behavior, variable, vector) {
        let explorerWindowLabel = "explorer-window:" + Date.now()

        WindowsEndpoints.openExplorerWindow(explorerWindowLabel)
            .then(() => {
                const newExplorerWindow = TAURI.window.WebviewWindow.getByLabel(explorerWindowLabel)

                // Wait until the new explorer window is initialized
                newExplorerWindow.once('ready', () => {
                    // Emit to get stability attractors
                    newExplorerWindow.emit('get-stability-attractors', {
                        node: node,
                        behavior: behavior,
                        variable: variable,
                        vector: vector,
                        windowSessionKey: Computation.getWindowSessionKey()
                    })
                })
            }).catch((errorMessage) => {
            Dialog.errorMessage(errorMessage)
        })
    },


    openTreeExplorerWindow() {
        let treeWindowLabel = "tree-window:" + Date.now()
        let windowTitle = Computation.getModelTitle() + ", started: " + new Date(Computation.getWindowTimestamp()).toLocaleTimeString('en-GB')

        WindowsEndpoints.openTreeExplorerWindow(treeWindowLabel, windowTitle)
            .then(() => {
                const newTreeWindow = TAURI.window.WebviewWindow.getByLabel(treeWindowLabel)

                // Wait until the new tree explorer window is initialized
                newTreeWindow.once('ready', () => {
                    // Emit to send window session key
                    newTreeWindow.emit('send-window-session-key', { windowSessionKey: Computation.getWindowSessionKey() })
                })
            }).catch((errorMessage) => {
            Dialog.errorMessage(errorMessage)
        })
    },

}