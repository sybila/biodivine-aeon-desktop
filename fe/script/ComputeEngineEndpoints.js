let ComputeEngineEndpoints = {

    validateUpdateFunction(modelFragment, callback) {
        window.__TAURI__.invoke('check_update_function', { data: modelFragment })
            .then((responseOk) => {
                let responseObject = JSON.parse(responseOk);
                return callback(undefined, responseObject);
            })
            .catch((responseError) => {
                let responseObject = JSON.parse(responseError);
                return callback(responseObject, undefined)
            });
    }
}


