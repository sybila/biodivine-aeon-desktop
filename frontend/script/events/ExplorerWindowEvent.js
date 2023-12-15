// Listen for 'get-attractors' event to show attractors in explorer window
TAURI.event.listen("get-attractors", (event) => {
	Computation.setSessionKey(event.payload["sessionKey"]);

	const behavior = event.payload["behavior"];

	UI.isLoading(true);
	ComputationResultsCommands.getAttractors(behavior)
		.then((okJson) => {
			UI.isLoading(false);
			showInExplorer(okJson);
		})
		.catch((errorMessage) => {
			UI.isLoading(false);
			Dialog.errorMessage(errorMessage);
		});
});


// Listen for 'get-tree-attractors' event to show attractors in explorer window
TAURI.event.listen("get-tree-attractors", (event) => {
	Computation.setSessionKey(event.payload["sessionKey"]);

	const node = event.payload["node"];

	UI.isLoading(true);
	ComputationResultsCommands.getTreeAttractors(node)
		.then((okJson) => {
			UI.isLoading(false);
			showInExplorer(okJson);
		})
		.catch((errorMessage) => {
			UI.isLoading(false);
			Dialog.errorMessage(errorMessage);
		});
});


// Listen for 'get-stability-attractors' event to show attractors in explorer window
TAURI.event.listen("get-stability-attractors", (event) => {
	Computation.setSessionKey(event.payload["sessionKey"]);

	const node = event.payload["node"];
	const behavior = event.payload["behavior"];
	const variable = event.payload["variable"];
	const vector = event.payload["vector"];

	UI.isLoading(true);
	ComputationResultsCommands.getStabilityAttractors(node, behavior, variable, vector)
		.then((okJson) => {
			UI.isLoading(false);
			showInExplorer(okJson);
		})
		.catch((errorMessage) => {
			UI.isLoading(false);
			Dialog.errorMessage(errorMessage);
		});
});
