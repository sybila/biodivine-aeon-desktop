/*
    Responsible for calling Tauri commands dealing with .aeon and .sbml model.
    Commands return Promises.
 */
let ModelCommands = {

	checkUpdateFunction(modelFragment) {
		return TAURI.core.invoke("check_update_function", {
			data: modelFragment
		});
	},

	sbmlToAeon(sbmlString) {
		return TAURI.core.invoke("sbml_to_aeon", {
			sbmlString: sbmlString
		});
	},

	bnetToAeon(bnetString) {
		return TAURI.core.invoke("bnet_to_aeon", {
			bnetString: bnetString
		})
	},

	aeonToSbml(aeonString) {
		return TAURI.core.invoke("aeon_to_sbml", {
			aeonString: aeonString
		});
	},

	aeonToSbmlInstantiated(aeonString) {
		return TAURI.core.invoke("aeon_to_sbml_instantiated", {
			aeonString: aeonString
		});
	},
};
