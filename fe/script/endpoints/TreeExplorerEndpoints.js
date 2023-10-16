let TreeExplorerEndpoints = {

    autoExpandBifurcationTree(nodeId, depth) {
        return TAURI.invoke('auto_expand', {
            nodeId: nodeId,
            depth: depth,
            windowSessionKey: Computation.getWindowSessionKey()
        })
    },

    getDecisionAttributes(nodeId) {
        return TAURI.invoke('get_attributes', {
            nodeId: nodeId,
            windowSessionKey: Computation.getWindowSessionKey()
        })
    },

    applyTreePrecision(precision) {
        return TAURI.invoke('apply_tree_precision', {
            precision: precision,
            windowSessionKey: Computation.getWindowSessionKey()
        })
    },

    getTreePrecision() {
        return TAURI.invoke('get_tree_precision', {
            windowSessionKey: Computation.getWindowSessionKey()
        })
    },

    selectDecisionAttribute(nodeId, attrId) {
        return TAURI.invoke('apply_attribute', {
            nodeId: nodeId,
            attributeId: attrId,
            windowSessionKey: Computation.getWindowSessionKey()
        })
    },

    deleteDecision(nodeId) {
        return TAURI.invoke('revert_decision', {
            nodeId: nodeId,
            windowSessionKey: Computation.getWindowSessionKey()
        })
    },

    getStabilityData(nodeId, behaviour) {
        return TAURI.invoke('get_stability_data', {
            nodeId: nodeId.toString(),
            behaviourStr: behaviour,
            windowSessionKey: Computation.getWindowSessionKey()
        })
    },

}
