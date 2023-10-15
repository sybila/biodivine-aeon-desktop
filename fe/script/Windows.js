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

    async newWitnessWindow(witness) {
        const witnessWindowLabel = await this.openModelInNewWindow(witness)
        if (witnessWindowLabel !== null) {
            const witnessWindow = TAURI.window.WebviewWindow.getByLabel(witnessWindowLabel)
            // Emit to open model editor tab in new witness window
            witnessWindow.emit('open-editor-tab', {})

            // // Wait until the new window is initialized
            // witnessWindow.once('ready', () => {
            //
            // })
        }
    },

    openWitnessWindow(witness) {
        ComputationResultsEndpoints.getWitness(witness)
            .then((witness) => {
                this.newWitnessWindow(witness)
            })
            .catch((errorMessage) => {
                MessageDialog.errorMessage(errorMessage)
            })
    },

    openTreeWitnessWindow(node) {
        ComputationResultsEndpoints.getTreeWitness(node)
            .then((witness) => {
                this.newWitnessWindow(witness)
            })
            .catch((errorMessage) => {
                MessageDialog.errorMessage(errorMessage)
            })
    },

    openStabilityWitnessWindow(node, behavior, variable, vector) {
        ComputationResultsEndpoints.getStabilityWitness(node, behavior, variable, vector)
            .then((witness) => {
                this.newWitnessWindow(witness)
            })
            .catch((errorMessage) => {
                MessageDialog.errorMessage(errorMessage)
            })
    },

    openAttractorExplorerWindow(behavior) {
        let explorerWindowLabel = "explorer-window:" + Date.now()

        TAURI.invoke('open_explorer_window', {
            label: explorerWindowLabel
        })
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
                MessageDialog.errorMessage(errorMessage)
            })
    },

    openTreeAttractorExplorerWindow(node) {
        let explorerWindowLabel = "explorer-window:" + Date.now()

        TAURI.invoke('open_explorer_window', {
            label: explorerWindowLabel
        })
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
            MessageDialog.errorMessage(errorMessage)
        })
    },

    openStabilityAttractorExplorerWindow(node, behavior, variable, vector) {
        let explorerWindowLabel = "explorer-window:" + Date.now()

        TAURI.invoke('open_explorer_window', {
            label: explorerWindowLabel
        })
            .then(() => {
                const newExplorerWindow = TAURI.window.WebviewWindow.getByLabel(explorerWindowLabel)

                // Wait until the new explorer window is initialized
                newExplorerWindow.once('ready', () => {
                    // Emit to get tree attractors
                    newExplorerWindow.emit('get-stability-attractors', {
                        node: node,
                        behavior: behavior,
                        variable: variable,
                        vector: vector,
                        windowSessionKey: Computation.getWindowSessionKey()
                    })
                })
            }).catch((errorMessage) => {
            MessageDialog.errorMessage(errorMessage)
        })
    },


    openTreeExplorerWindow() {
        let treeWindowLabel = "tree-window:" + Date.now()

        TAURI.invoke('open_tree_explorer_window', {
            label: treeWindowLabel
        })
            .then(() => {
                console.log("tree window opened")
                const newTreeWindow = TAURI.window.WebviewWindow.getByLabel(treeWindowLabel)

                // Wait until the new explorer window is initialized
                newTreeWindow.once('ready', () => {
                    // Emit to get attractors
                    newTreeWindow.emit('send-window-session-key', { windowSessionKey: Computation.getWindowSessionKey() })
                })
            }).catch((errorMessage) => {
            MessageDialog.errorMessage(errorMessage)
        })
    },

}