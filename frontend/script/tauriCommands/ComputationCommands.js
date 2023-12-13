/*
    Responsible for calling Tauri commands dealing with Computation.
    Commands return Promises.
 */
let ComputationCommands = {

    startComputation(aeonString, sessionKey) {
         return TAURI.invoke('start_computation', {
            sessionKey: sessionKey,
            aeonString: aeonString
        })
    },

    getComputationProcessInfo() {
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