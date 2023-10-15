let ComputationResultsEndpoints = {

    // Witness
    getWitness(witness) {
        return TAURI.invoke('get_witness', {
            classStr: witness,
            windowSessionKey: Computation.getWindowSessionKey()
        })
    },

    getTreeWitness(node) {
        return TAURI.invoke('get_tree_witness', {
            nodeId: node,
            windowSessionKey: Computation.getWindowSessionKey()
        })
    },

    getStabilityWitness(node, behavior, variable, vector) {
        return TAURI.invoke('get_stability_witness', {
            nodeId: node,
            behaviourStr: behavior,
            variableStr: variable,
            vectorStr: vector,
            windowSessionKey: Computation.getWindowSessionKey()
        })
    },


    // Attractor
    getAttractors(behavior) {
        return TAURI.invoke('get_attractors', {
            classStr: behavior,
            windowSessionKey: Computation.getWindowSessionKey()
        })
    },

    getTreeAttractors(node) {
        return TAURI.invoke('get_tree_attractors', {
            nodeId: node,
            windowSessionKey: Computation.getWindowSessionKey()
        })
    },

    getStabilityAttractors(node, behavior, variable, vector) {
        return TAURI.invoke('get_stability_attractors', {
            nodeId: node,
            behaviourStr: behavior,
            variableStr: variable,
            vectorStr: vector,
            windowSessionKey: Computation.getWindowSessionKey()
        })
    },


    // Tree
    getBifurcationTree() {
        return TAURI.invoke('get_bifurcation_tree', {
            windowSessionKey: Computation.getWindowSessionKey()
        })
    }
}
