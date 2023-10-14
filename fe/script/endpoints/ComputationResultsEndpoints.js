let ComputationResultsEndpoints = {

    getWitness(witness) {
        return TAURI.invoke('get_witness', {
            classStr: witness,
            windowSessionKey: Computation.getWindowSessionKey()
        })
    },

    getAttractors(behavior) {
        return TAURI.invoke('get_attractors', {
            classStr: behavior,
            windowSessionKey: Computation.getWindowSessionKey()
        })
    }
}