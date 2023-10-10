let Computation = {
    _windowSessionKey: undefined,
    // a timestamp of last successfully started computation
    // if status returns a different timestamp, we know results are out of date
    _lastComputation: undefined,

    setActiveComputation(timestamp) {
        if (this._lastComputation !== timestamp) {
            // if timestamp changed, switch to undefined.
            this._lastComputation = undefined;
        }
    },

    isActiveComputation() {
        return this._lastComputation !== undefined;
    },

    startComputation(aeonString, windowSessionKey) {
        this._windowSessionKey = windowSessionKey

        TAURI.invoke('start_computation', {
            windowSessionKey: windowSessionKey,
            aeonString: aeonString
        })
            .then((responseOk) => {
                let responseObject = JSON.parse(responseOk)
                let resultObject = JSON.parse(responseObject['result'])
                console.log("Started computation ", resultObject.timestamp)
                this._lastComputation = resultObject.timestamp
                this.update_computation_process()
            })
            .catch((responseError) => {
                let errorObject = JSON.parse(responseError)
                MessageDialog.errorMessage(errorObject['message'])
            });
    },


    update_computation_process() {
        TAURI.invoke('get_computation_process_info', {
            windowSessionKey: this._windowSessionKey
        })
            .then((response) => {
                const responseObject = JSON.parse(response)

                // Update UI of Computation Window
                if (typeof UI !== 'undefined') {
                    UI.updateComputationStatus(true, responseObject);
                }

                // Update recursively again if computation is still running
                if (responseObject["is_running"]) {
                    setTimeout(() => { this.update_computation_process(); }, 1000)
                }
        })
    },

    getResults(callback) {
        TAURI.invoke('get_results', {
            windowSessionKey: this._windowSessionKey
        })
            .then((responseOK) => {
                let responseOkObject = JSON.parse(responseOK)
                let resultObject = JSON.parse(responseOkObject['result'])
                return callback(undefined, resultObject)
            })
            .catch((responseErr) => {
                let responseErrorObject = JSON.parse(responseErr)
                return callback(responseErrorObject['message'], undefined)
            })
    },

    cancelComputation() {
        TAURI.invoke('cancel_computation', {
            windowSessionKey: this._windowSessionKey
        })
            .catch((errorMessage) => {
                MessageDialog.errorMessage(errorMessage)
            });
    },
}