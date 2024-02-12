/*
    Responsible for calling Tauri commands dealing with bifurcation decision tree.
    Commands return Promises.
 */
let TreeExplorerCommands = {

	autoExpandBifurcationTree(nodeId, depth) {
		return TAURI.core.invoke("auto_expand", {
			nodeId: nodeId,
			depth: depth,
			sessionKey: Computation.getSessionKey()
		});
	},

	getDecisionAttributes(nodeId) {
		return TAURI.core.invoke("get_attributes", {
			nodeId: nodeId,
			sessionKey: Computation.getSessionKey()
		});
	},

	applyTreePrecision(precision) {
		return TAURI.core.invoke("apply_tree_precision", {
			precision: precision,
			sessionKey: Computation.getSessionKey()
		});
	},

	getTreePrecision() {
		return TAURI.core.invoke("get_tree_precision", {
			sessionKey: Computation.getSessionKey()
		});
	},

	applyDecisionAttribute(nodeId, attrId) {
		return TAURI.core.invoke("apply_attribute", {
			nodeId: nodeId,
			attributeId: attrId,
			sessionKey: Computation.getSessionKey()
		});
	},

	revertDecision(nodeId) {
		return TAURI.core.invoke("revert_decision", {
			nodeId: nodeId,
			sessionKey: Computation.getSessionKey()
		});
	},

	getStabilityData(nodeId, behaviour) {
		return TAURI.core.invoke("get_stability_data", {
			nodeId: nodeId.toString(),
			behaviourStr: behaviour,
			sessionKey: Computation.getSessionKey()
		});
	},

};
