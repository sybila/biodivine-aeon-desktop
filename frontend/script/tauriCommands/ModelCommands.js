/*
    Responsible for calling Tauri commands dealing with .aeon and .sbml model.
    Commands return Promises.
 */
let ModelCommands = {

    checkUpdateFunction(modelFragment) {
        return TAURI.invoke('check_update_function', {
            data: modelFragment
        })
    },

    sbmlToAeon(sbmlString) {
        return TAURI.invoke('sbml_to_aeon', {
            sbmlString: sbmlString
        })
    },

    aeonToSbml(aeonString) {
        return TAURI.invoke('aeon_to_sbml', {
            aeonString: aeonString
        })
    },

    aeonToSbmlInstantiated(aeonString) {
        return TAURI.invoke('aeon_to_sbml_instantiated', {
            aeonString: aeonString
        })
    },
}
