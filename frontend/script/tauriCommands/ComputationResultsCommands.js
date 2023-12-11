/*
    Responsible for calling Tauri commands dealing with results of Computation.
    Commands return Promises.
 */
let ComputationResultsCommands = {

    // Witness
    getWitness(witness) {
        return TAURI.invoke('get_witness', {
            classStr: witness,
            sessionKey: Computation.getSessionKey()
        })
    },

    getTreeWitness(node) {
        return TAURI.invoke('get_tree_witness', {
            nodeId: node,
            sessionKey: Computation.getSessionKey()
        })
    },

    getStabilityWitness(node, behavior, variable, vector) {
        return TAURI.invoke('get_stability_witness', {
            nodeId: node,
            behaviourStr: behavior,
            variableStr: variable,
            vectorStr: vector,
            sessionKey: Computation.getSessionKey()
        })
    },


    // Attractor
    getAttractors(behavior) {
        return TAURI.invoke('get_attractors', {
            classStr: behavior,
            sessionKey: Computation.getSessionKey()
        })
    },

    getTreeAttractors(node) {
        return TAURI.invoke('get_tree_attractors', {
            nodeId: node,
            sessionKey: Computation.getSessionKey()
        })
    },

    getStabilityAttractors(node, behavior, variable, vector) {
        return TAURI.invoke('get_stability_attractors', {
            nodeId: node,
            behaviourStr: behavior,
            variableStr: variable,
            vectorStr: vector,
            sessionKey: Computation.getSessionKey()
        })
    },


    // Tree
    getBifurcationTree() {
        return TAURI.invoke('get_bifurcation_tree', {
            sessionKey: Computation.getSessionKey()
        })
    }
}
