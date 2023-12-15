/*
    Responsible for calling Tauri commands dealing with bifurcation decision tree.
    Commands return Promises.
 */
let TreeExplorerCommands = {

	autoExpandBifurcationTree(nodeId, depth) {
		return TAURI.invoke("auto_expand", {
			nodeId: nodeId,
			depth: depth,
			sessionKey: Computation.getSessionKey()
		});
	},

	getDecisionAttributes(nodeId) {
		return TAURI.invoke("get_attributes", {
			nodeId: nodeId,
			sessionKey: Computation.getSessionKey()
		});
	},

	applyTreePrecision(precision) {
		return TAURI.invoke("apply_tree_precision", {
			precision: precision,
			sessionKey: Computation.getSessionKey()
		});
	},

	getTreePrecision() {
		return TAURI.invoke("get_tree_precision", {
			sessionKey: Computation.getSessionKey()
		});
	},

	applyDecisionAttribute(nodeId, attrId) {
		return TAURI.invoke("apply_attribute", {
			nodeId: nodeId,
			attributeId: attrId,
			sessionKey: Computation.getSessionKey()
		});
	},

	revertDecision(nodeId) {
		return TAURI.invoke("revert_decision", {
			nodeId: nodeId,
			sessionKey: Computation.getSessionKey()
		});
	},

	getStabilityData(nodeId, behaviour) {
		return TAURI.invoke("get_stability_data", {
			nodeId: nodeId.toString(),
			behaviourStr: behaviour,
			sessionKey: Computation.getSessionKey()
		});
	},

};
