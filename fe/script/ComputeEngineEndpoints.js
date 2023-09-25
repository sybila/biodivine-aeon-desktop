let ComputeEngineEndpoints = {

    invokeComputeEngineEndpoint(endpointName, data, callback) {
        TAURI.invoke(endpointName, { data: data })
            .then((responseOk) => {
                let responseOkObject = JSON.parse(responseOk);
                return callback(undefined, responseOkObject);
            })
            .catch((responseError) => {
                let responseErrorObject = JSON.parse(responseError);
                return callback(responseErrorObject, undefined)
            });
    },

    getCurrentWindowLabel() {
        return TAURI.window.getCurrent().label;
    },

    validateUpdateFunction(modelFragment, callback) {
        this.invokeComputeEngineEndpoint('check_update_function', modelFragment, callback);
    },

    sbmlToAeon(sbmlString, callback) {
        this.invokeComputeEngineEndpoint('sbml_to_aeon', sbmlString, callback);
    },

    aeonToSbml(aeonString, callback) {
        this.invokeComputeEngineEndpoint('aeon_to_sbml', aeonString, callback);
    },

    aeonToSbmlInstantiated(aeonString, callback) {
        this.invokeComputeEngineEndpoint('aeon_to_sbml_instantiated', aeonString, callback)
    },

    startComputation(aeonString) {
        if (aeonString === undefined) {
            MessageDialog.errorMessage("Empty model.")
            return undefined;
        }
        this.waitingForResult = true;
        const currentWindowLabel = this.getCurrentWindowLabel();
        TAURI.invoke('start_computation', { windowLabel: currentWindowLabel, aeonString: aeonString })
            .then((responseOk) => {
                let responseObject = JSON.parse(responseOk)
                let resultObject = JSON.parse(responseObject['result'])
                console.log("Started computation ", resultObject.timestamp);
                this._lastComputation = resultObject.timestamp;
            })
            .catch((responseError) => {
                let errorObject = JSON.parse(responseError)
                MessageDialog.errorMessage(errorObject['message'])
            });
    },

    cancelComputation() {
        const currentWindowLabel = this.getCurrentWindowLabel();
        TAURI.invoke('cancel_computation', { windowLabel: currentWindowLabel })
            .then((responseOK) => {
                let responseOkObject = JSON.parse(responseOK)
                MessageDialog.infoMessage(responseOkObject['result'])
            })
            .catch((responseError) => {
                let responseErrorObject = JSON.parse(responseError)
                MessageDialog.errorMessage(responseErrorObject['message'])
            });
    },

    // For now, test by calling it from console
    getResults() {
        const currentWindowLabel = this.getCurrentWindowLabel();
        TAURI.invoke('get_results', { windowLabel: currentWindowLabel })
            .then((responseOK) => {
                let responseOkObject = JSON.parse(responseOK)
                console.log(responseOkObject)
            })
            .catch((responseError) => {
                let responseErrorObject = JSON.parse(responseError)
                console.log(responseErrorObject)
            });
    },
}
