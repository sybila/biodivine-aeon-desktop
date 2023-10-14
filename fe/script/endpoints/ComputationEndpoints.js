let ComputationEndpoints = {

    startComputation(aeonString, windowSessionKey) {
        Computation.setWindowSessionKey(windowSessionKey)

        TAURI.invoke('start_computation', {
            windowSessionKey: windowSessionKey,
            aeonString: aeonString
        })
            .then((responseOk) => {
                let responseObject = JSON.parse(responseOk)
                let resultObject = JSON.parse(responseObject['result'])
                console.log("Started computation ", resultObject.timestamp)
                Computation.setLastComputation(resultObject.timestamp)
                this.update_computation_process()
            })
            .catch((responseError) => {
                let errorObject = JSON.parse(responseError)
                MessageDialog.errorMessage(errorObject['message'])
            });
    },

    update_computation_process() {
        TAURI.invoke('get_computation_process_info', {
            windowSessionKey: Computation.getWindowSessionKey()
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
            windowSessionKey: Computation.getWindowSessionKey()
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
            windowSessionKey: Computation.getWindowSessionKey()
        })
            .catch((errorMessage) => {
                MessageDialog.errorMessage(errorMessage)
            });
    },
}