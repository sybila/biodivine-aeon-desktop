/*
    Responsible for calling Tauri commands dealing with Computation.
    Commands return Promises.
 */
let ComputationCommands = {

	startComputation(aeonString, sessionKey) {
		return TAURI.core.invoke("start_computation", {
			sessionKey: sessionKey,
			aeonString: aeonString
		});
	},

	getComputationProcessInfo() {
		return TAURI.core.invoke("get_computation_process_info", {
			sessionKey: Computation.getSessionKey()
		});
	},

	getResults() {
		return TAURI.core.invoke("get_results", {
			sessionKey: Computation.getSessionKey()
		});
	},

	cancelComputation() {
		TAURI.core.invoke("cancel_computation", {
			sessionKey: Computation.getSessionKey()
		})
			.catch((errorMessage) => {
				Dialog.errorMessage(errorMessage);
			});
	},
};