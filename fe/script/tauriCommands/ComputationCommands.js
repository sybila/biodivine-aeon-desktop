let ComputationCommands = {

    startComputation(aeonString, sessionKey) {
         return TAURI.invoke('start_computation', {
            sessionKey: sessionKey,
            aeonString: aeonString
        })
    },

    update_computation_process() {
        return TAURI.invoke('get_computation_process_info', {
            sessionKey: Computation.getSessionKey()
        })
    },

    getResults() {
        return TAURI.invoke('get_results', {
            sessionKey: Computation.getSessionKey()
        })
    },

    cancelComputation() {
        TAURI.invoke('cancel_computation', {
            sessionKey: Computation.getSessionKey()
        })
            .catch((errorMessage) => {
                Dialog.errorMessage(errorMessage)
            });
    },
}