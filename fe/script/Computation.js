let Computation = {
    _windowSessionKey: undefined,
    _lastComputation: undefined,
    _modelTitle: undefined,
    _windowTimestamp: undefined,

    setWindowSessionKey(windowSessionKey) {
        this._windowSessionKey = windowSessionKey
    },

    getWindowSessionKey() {
        return this._windowSessionKey
    },

    setLastComputation(timestamp) {
        this._lastComputation = timestamp
    },

    setActiveComputation(timestamp) {
        if (this._lastComputation !== timestamp) {
            // if timestamp changed, switch to undefined.
            this._lastComputation = undefined;
        }
    },

    setModelTitle(modelTitle) {
        this._modelTitle = modelTitle
    },

    getModelTitle() {
        return this._modelTitle
    },

    setWindowTimestamp(timestamp) {
        this._windowTimestamp = timestamp
    },

    getWindowTimestamp() {
        return this._windowTimestamp
    },

    isActiveComputation() {
        return this._lastComputation !== undefined;
    },

    update_computation_process() {
        ComputationEndpoints.update_computation_process()
            .then((computationInfoObject) => {
                // Update UI of Computation Window
                if (typeof UI !== 'undefined') {
                    UI.updateComputationStatus(true, computationInfoObject);
                }

                // Update recursively again if computation is still running
                if (computationInfoObject["is_running"]) {
                    setTimeout(() => { this.update_computation_process(); }, 1000)
                }
            })
            .catch((errorMessage) => {
                Dialog.errorMessage(errorMessage)
            })
    },
}
