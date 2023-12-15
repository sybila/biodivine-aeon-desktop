/*
    Handles what happens when new window is created/opened.
 */
let Windows = {

    // Open empty model editor window.
    openNewModelEditorWindow() {
        let windowLabel = 'model-window:' + Date.now()
        WindowsCommands.openModelWindow(windowLabel)
    },

    // Open model in new window and return label of this window
    async openModelInNewWindow(modelString) {
        let windowLabel = 'model-window:' + Date.now()

        await WindowsCommands.openModelWindow(windowLabel)
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

    // Open computation window and start computation
    openComputationWindow(aeonString) {
        let timestamp = Date.now()
        let windowLabel = 'computation-window:' + timestamp

        let modelTitle = Model.getModelName(aeonString)
        if (modelTitle === undefined || modelTitle.length < 1) {
            let filePath = ModelEditor.getModelFilePath()
            modelTitle = filePath !== undefined ? filePath : "Model without name"
        }

        let windowTitle = modelTitle + ", computation started: " + new Date(timestamp).toLocaleTimeString('en-GB')

        WindowsCommands.openComputationWindow(windowLabel, windowTitle)
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

    // Open witness model in a new window and open model editor tab
    async openWitnessInNewWindow(witnessModelString) {
        let windowLabel = 'witness-window:' + Date.now()
        let windowTitle = await TAURI.window.getCurrent().title()
        windowTitle = "Witness of " + windowTitle

        WindowsCommands.openWitnessWindow(windowLabel, windowTitle)
            .then(() => {
                const newWitnessWindow = TAURI.window.WebviewWindow.getByLabel(windowLabel)

                // Wait until the new window is initialized
                newWitnessWindow.once('ready', () => {

                    // Emit model string to the new window
                    newWitnessWindow.emit('import-model', { modelString: witnessModelString })

                    // Open model editor tab in new witness window
                    newWitnessWindow.emit('open-editor-tab', {})
                })
            }).catch((errorMessage) => {
                Dialog.errorMessage(errorMessage)
                windowLabel = null
            })
    },

    // Get witness from computation and try to open it in a new window
    openWitnessWindow(witness) {
        UI.isLoading(true)
        ComputationResultsCommands.getWitness(witness)
            .then((witness) => {
                UI.isLoading(false)
                this.openWitnessInNewWindow(witness)
            })
            .catch((errorMessage) => {
                UI.isLoading(false)
                Dialog.errorMessage(errorMessage)
            })
    },

    // Get witness from tree explorer and try to open it in new window
    openTreeWitnessWindow(node) {
        UI.isLoading(true)
        ComputationResultsCommands.getTreeWitness(node)
            .then((witness) => {
                UI.isLoading(false)
                this.openWitnessInNewWindow(node)
            })
            .catch((errorMessage) => {
                UI.isLoading(false)
                Dialog.errorMessage(errorMessage)
            })
    },

    // Get witness from stability analysis and try to open it in new window
    openStabilityWitnessWindow(node, behavior, variable, vector) {
        UI.isLoading(true)
        ComputationResultsCommands.getStabilityWitness(node, behavior, variable, vector)
            .then((witness) => {
                UI.isLoading(false)
                this.openWitnessInNewWindow(witness)
            })
            .catch((errorMessage) => {
                UI.isLoading(false)
                Dialog.errorMessage(errorMessage)
            })
    },

    async openAttractorExplorerWindow(behavior) {
        let explorerWindowLabel = "explorer-window:" + Date.now()
        let explorerWindowTitle = await TAURI.window.getCurrent().title()
        explorerWindowTitle = "Attractor of " + explorerWindowTitle

        WindowsCommands.openExplorerWindow(explorerWindowLabel, explorerWindowTitle)
            .then(() => {
                const newExplorerWindow = TAURI.window.WebviewWindow.getByLabel(explorerWindowLabel)

                // Wait until the new explorer window is initialized
                newExplorerWindow.once('ready', () => {
                    // Emit to get attractors
                    newExplorerWindow.emit('get-attractors', {
                        behavior: behavior,
                        sessionKey: Computation.getSessionKey()
                    })
                })
            }).catch((errorMessage) => {
                Dialog.errorMessage(errorMessage)
            })
    },

    async openTreeAttractorExplorerWindow(node) {
        let explorerWindowLabel = "explorer-window:" + Date.now()
        let explorerWindowTitle = await TAURI.window.getCurrent().title()
        explorerWindowTitle = "Attractor of " + explorerWindowTitle

        WindowsCommands.openExplorerWindow(explorerWindowLabel, explorerWindowTitle)
            .then(() => {
                const newExplorerWindow = TAURI.window.WebviewWindow.getByLabel(explorerWindowLabel)

                // Wait until the new explorer window is initialized
                newExplorerWindow.once('ready', () => {
                    // Emit to get tree attractors
                    newExplorerWindow.emit('get-tree-attractors', {
                        node: node,
                        sessionKey: Computation.getSessionKey()
                    })
                })
            }).catch((errorMessage) => {
            Dialog.errorMessage(errorMessage)
        })
    },

    async openStabilityAttractorExplorerWindow(node, behavior, variable, vector) {
        let explorerWindowLabel = "explorer-window:" + Date.now()
        let explorerWindowTitle = await TAURI.window.getCurrent().title()
        explorerWindowTitle = "Attractor of " + explorerWindowTitle

        WindowsCommands.openExplorerWindow(explorerWindowLabel, explorerWindowTitle)
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
                        sessionKey: Computation.getSessionKey()
                    })
                })
            }).catch((errorMessage) => {
            Dialog.errorMessage(errorMessage)
        })
    },


    openTreeExplorerWindow() {
        let treeWindowLabel = Computation.getTreeExplorerWindowLabel()

        // If the window is already opened, just focus on it
        if (treeWindowLabel !== undefined) {
            const treeWindow = TAURI.window.WebviewWindow.getByLabel(treeWindowLabel)
            treeWindow.setFocus()
            return
        }

        treeWindowLabel = "tree-window:" + Date.now()
        Computation.setTreeExplorerWindowLabel(treeWindowLabel)
        let windowTitle = Computation.getModelTitle() + ", computation started: " + new Date(Computation.getWindowTimestamp()).toLocaleTimeString('en-GB')

        WindowsCommands.openTreeExplorerWindow(treeWindowLabel, windowTitle)
            .then(() => {
                const newTreeWindow = TAURI.window.WebviewWindow.getByLabel(treeWindowLabel)

                // Wait until the new tree explorer window is initialized
                newTreeWindow.once('ready', () => {
                    // Emit to send window session key
                    newTreeWindow.emit('send-window-session-key', { sessionKey: Computation.getSessionKey() })
                })
            }).catch((errorMessage) => {
            Dialog.errorMessage(errorMessage)
        })
    },

    openHelpWindow() {
        const helpWindow = TAURI.window.WebviewWindow.getByLabel("help-window")

        // If the window is already opened, just focus on it
        if (helpWindow !== null) {
            helpWindow.setFocus()
            return
        }

        WindowsCommands.openHelpWindow()
    },

    openManualWindow() {
        const manualWindow = TAURI.window.WebviewWindow.getByLabel("manual-window")

        // If the window is already opened, just focus on it
        if (manualWindow !== null) {
            manualWindow.setFocus()
            return
        }

        WindowsCommands.openManualWindow()
    },
}