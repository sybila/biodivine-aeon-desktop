let ComputationEndpoints = {

    startComputation(aeonString, windowSessionKey) {
         return TAURI.invoke('start_computation', {
            windowSessionKey: windowSessionKey,
            aeonString: aeonString
        })
    },

    update_computation_process() {
        return TAURI.invoke('get_computation_process_info', {
            windowSessionKey: Computation.getWindowSessionKey()
        })
    },

    getResults() {
        return TAURI.invoke('get_results', {
            windowSessionKey: Computation.getWindowSessionKey()
        })
    },

    cancelComputation() {
        TAURI.invoke('cancel_computation', {
            windowSessionKey: Computation.getWindowSessionKey()
        })
            .catch((errorMessage) => {
                Dialog.errorMessage(errorMessage)
            });
    },
}