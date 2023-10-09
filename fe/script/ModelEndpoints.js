let ModelEndpoints = {

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
}
