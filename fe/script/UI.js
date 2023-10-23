let ContentTabs = {
	engine: "tab-engine",
	modelEditor: "tab-model-editor",
	results: "tab-results",
}

const DOUBLE_CLICK_DELAY = 400;

/*
	Allows access to operations with the global UI (i.e. operating the menus, showing content panels, etc.).
*/
let UI = {

	// Element where the cytoscape editor resides.
	cytoscapeEditor: undefined,
	// Element of the menu that is displayed for each node/edge when selected.
	_nodeMenu: undefined,
	_edgeMenu: undefined,
	// Contains pairs of elements of the form { button: ..., tab: ... } corresponding to the side menu.
	_tabsAndButtons: undefined,



	init: function() {
		this.cytoscapeEditor = document.getElementById("cytoscape-editor");		
		this._nodeMenu = document.getElementById("node-menu");
		this._edgeMenu = document.getElementById("edge-menu");
		
		let sideMenu = document.getElementById("side-menu");
		let sideMenuButtons = sideMenu.getElementsByClassName("button");
		this._tabsAndButtons = [];
		for (var i = 0; i < sideMenuButtons.length; i++) {
			let button = sideMenuButtons[i];
			let tab = document.getElementById(button.getAttribute("tab-id"));
			this._tabsAndButtons.push({ tab: tab, button: button });
		}
		
		this._initNodeMenu(this._nodeMenu);
		this._initEdgeMenu(this._edgeMenu);
		this._initSideMenu(sideMenu);	

		// Init fake button (sadly we have to do this explicitly
		// because it is not a proper menu button).
		// Show hint popup on mouse enter when button is not selected.
		let fakeButtonGroups = document.getElementsByClassName("button-group-fake");
		for (let group of fakeButtonGroups) {
			let button = group.getElementsByClassName("button-fake")[0];
			let hint = group.getElementsByClassName("hint")[0];		
			button.addEventListener("mouseenter", (e) => {
				let selected = button.classList.contains("selected");
				if (!selected) {
					group.style.width = "272px";
					hint.classList.remove("invisible");
				}				
			});
			// Hide hint popup on mouse leave
			button.addEventListener("mouseleave", (e) => {
				group.style.width = "72px";				
				hint.classList.add("invisible");			
			});
		}		
	},

	openHelpWindow() {
		const helpWindow = TAURI.window.WebviewWindow.getByLabel("help-window")

		// If the window is already opened, just focus on it
		if (helpWindow !== null) {
			helpWindow.setFocus()
			return
		}

		new TAURI.window.WebviewWindow('help-window', {
			url: 'help-window.html',
			title: 'About & Help',
			width: 530,
			height: 700,
		})
	},

	updateComputationStatus(activeStatus, data) {
		let cmp = document.getElementById("computation");
		let cmpLabel = document.getElementById("compute-engine-status");
		let cmpStatus = document.getElementById("computation-status");
		let cmpProgress = document.getElementById("computation-progress");
		let cmpClasses = document.getElementById("computation-classes");
		let cmpCancel = document.getElementById("computation-cancel");
		let cmpDownload = document.getElementById("computation-download");
		let resultsWindow = document.getElementById("tab-results");

		// Reset classes
		cmpLabel.classList.remove("green", "orange");
		cmpStatus.classList.remove("red", "green", "orange");

		if (activeStatus) {
			if (data !== undefined) {
				// data about computation available
				let status = "(none)";
				// If there is a computation, it is probably running...
				if (data["timestamp"] !== null) {
					status = "running";
					// ...but, if it is cancelled, we are awaiting cancellation...
					if (data["is_cancelled"]) {
						status = "awaiting cancellation";
						resultsWindow.classList.add("gone");
					}
					// ...but, if it is not running, and it is not cancelled, then it must be done...
					if (!data["is_running"] && !data["is_cancelled"]) {
						status = "done";
					}
					// ...and, if it is not running, and it is cancelled, then it is actually cancelled.
					if (!data["is_running"] && data["is_cancelled"]) {
						status = "cancelled";
					}
				}

				// Update computation label color depending on current computation status.
				if (status === "(none)" || status === "done" || status === "cancelled") {
					cmpLabel.classList.add("green");
				} else {
					cmpLabel.classList.add("orange");
				}

				// Make status green/orange depending on state of computation.
				if (status === "done") {
					cmpStatus.classList.add("green");
				} else if (status !== "(none)") {
					cmpStatus.classList.add("orange");
				}

				// Progress is only shown when we are running...
				if (status === "running") {
					cmpProgress.parentElement.classList.remove("gone");
				} else {
					cmpProgress.parentElement.classList.add("gone");
				}
				cmp.classList.remove("gone");
				if (data.error !== null) {
					status += ", error: " + data.error;
				}
				cmpStatus.innerHTML = status;
				cmpProgress.textContent = data.progress;
				if (data["num_classes"] !== null) {
					cmpClasses.textContent = data["num_classes"];
				} else {
					cmpClasses.textContent = "-";
				}

				// Show cancel button if job is running and not cancelled 
				if (data["is_running"] && !data["is_cancelled"]) {
					cmpCancel.classList.remove("gone");
				} else {
					cmpCancel.classList.add("gone");
				}

				// Show download "partial" button if there is running or cancelled job
				if (data["is_running"] || data["is_cancelled"]) {
					cmpDownload.classList.remove("gone");
					cmpDownload.innerHTML = "Show partial result <img src=\"img/cloud_download-24px.svg\">";
				} else {
					cmpDownload.classList.add("gone");
				}

				if (data["timestamp"] !== undefined && Results.hasResults()) {
					// show warning if data is out of date
					Computation.setActiveComputation(data["timestamp"]);
					if (Computation.isActiveComputation()) {
						document.getElementById("results-expired").classList.add("gone");
					} else {
						document.getElementById("results-expired").classList.remove("gone");
					}
				} else {
					document.getElementById("results-expired").classList.add("gone");
				}			

				if (status === "done") {
					Results.show();
				}
			}
		} else {
			cmp.classList.add("gone");
		}
	},

	isEdgeMenuVisible() {
		return !this._edgeMenu.classList.contains("invisible");
	},

	isNodeMenuVisible() {
		return !this._nodeMenu.classList.contains("invisible");
	},

	// Close any content tab, if open.
	closeContent() {
		this.ensureContentTabOpen(undefined);
	},

	// A small utility method to show quick help.
	setQuickHelpVisible(visible) {
		if (visible || LiveModel.isEmpty()) {
			document.getElementById("quick-help").classList.remove("gone");
		} else {
			document.getElementById("quick-help").classList.add("gone");
		}
	},

	// Make sure the given content tab is open (for example because there is content in it that
	// needs to be seen).
	ensureContentTabOpen(tabId) {
		for (var i = 0; i < this._tabsAndButtons.length; i++) {
			let item = this._tabsAndButtons[i];
			if (item.tab.getAttribute("id") == tabId) {
				item.button.classList.add("selected");
				item.tab.classList.remove("gone");
			} else {
				item.button.classList.remove("selected");
				item.tab.classList.add("gone");
			}			
		}	
	},	


	// If given a position, show the center of the node menu at that position.
	// If no position is given, hide the menu.
	// ([Num, Num], Float = 1.0)
	toggleNodeMenu: function(position, zoom = 1.0) {
		let menu = this._nodeMenu;
		if (position === undefined) {
			menu.classList.add("invisible");			
			menu.style.left = "-100px";	// move it somewhere out of clickable area
			menu.style.top = "-100px";
		} else {
			menu.classList.remove("invisible");
			menu.style.left = position[0] + "px";
			menu.style.top = (position[1] + (60 * zoom)) + "px";
			// Scale applies current zoom, translate ensures the middle point of menu is 
			// actually at postion [left, top] (this makes it easier to align).
			// Note the magic constant next to zoom: It turns out we needed smaller font
			// size on the editor nodes (to make import more reasonable).
			// However, that made the menu much too big, so we are sticking with "zooming out"
			// the menu and keeping smaller sizes in the graph.
			menu.style.transform = "scale(" + (zoom * 0.75) + ") translate(-50%, -50%)";			
		}			
	},

	// Show the edge menu at the specified position with the provided data { observability, monotonicity }
	// If data or position is indefined, hide menu.
	toggleEdgeMenu(data, position, zoom = 1.0) {
		let menu = this._edgeMenu;
		if (position === undefined || data === undefined) {
			menu.classList.add("invisible");
			menu.style.left = "-100px";	// move it somewhere out of clickable area
			menu.style.top = "-100px";
		} else {
			menu.classList.remove("invisible");
			menu.style.left = position[0] + "px";
			menu.style.top = (position[1] + (60 * zoom)) + "px";
			// Scale applies current zoom, translate ensures the middle point of menu is 
			// actually at postion [left, top] (this makes it easier to align).			
			menu.style.transform = "scale(" + (zoom * 0.75) + ") translate(-50%, -50%)";
			menu.observabilityButton.updateState(data);
			menu.monotonicityButton.updateState(data);
		}
	},

	isLoading(status) {
		if (status) {
			document.getElementById("loading-indicator").classList.remove("invisible");
		} else {
			document.getElementById("loading-indicator").classList.add("invisible");
		}
	},

	// Trigger a download of Aeon exported file of the current model (if possible)
	downloadAeon() {
		let modelFile = LiveModel.exportAeon();
		if (modelFile === undefined) {
			Dialog.errorMessage(Strings.modelEmpty);
			return;
		}
		let filename = ModelEditor.getModelName();
        if (filename === undefined) {
        	filename = "model";
        }
        this._downloadFile(filename + ".aeon", modelFile)        
	},

	downloadSBML() {
		let aeonModel = LiveModel.exportAeon();
		if (aeonModel === undefined) {
			Dialog.errorMessage(Strings.modelEmpty)
			return;
		}
		let filename = ModelEditor.getModelName();
        if (filename === undefined) {
        	filename = "model";
        }

		ModelEndpoints.aeonToSbml(aeonModel)
			.then((sbmlModel) => {
				this._downloadFile(filename + ".sbml", sbmlModel);
			})
			.catch((errorMessage) => {
				Dialog.errorMessage(errorMessage)
			})
	},

	downloadSBMLInstantiated() {
		let aeonModel = LiveModel.exportAeon();
		if (aeonModel === undefined) {
			Dialog.errorMessage(Strings.modelEmpty);
			return;
		}
		let filename = ModelEditor.getModelName();
        if (filename === undefined) {
        	filename = "model";
        }

		ModelEndpoints.aeonToSbmlInstantiated(aeonModel)
			.then((sbmlModel) => {
				this._downloadFile(filename + "_instantiated.sbml", sbmlModel);
			})
			.catch((errorMessage) => {
				Dialog.errorMessage(errorMessage);
			})
	},

	async _downloadFile(name, content) {
		const saveLocation = await TAURI.dialog.save({
			defaultPath: name,
			title: 'Download model'
		});
		if (saveLocation !== null) {
			await TAURI.fs.writeTextFile(saveLocation, content);
		}
	},

	// Import Aeon file from the given file input element (if possible)
	async importAeon() {
		const filePath = await TAURI.dialog.open({
			multiple: false,
			directory: false,
			filters: [{
				name: 'AEON file',
				extensions: ['aeon', 'txt']
			}],
			title: 'Import .AEON'
		});
		if (!filePath || filePath.length === 0) {
			return;
		}

		ModelEditor.setModelFilePath(filePath)

		const aeonFileContent = await TAURI.fs.readTextFile(filePath);

		let error = await LiveModel.handleAeonModelImport(aeonFileContent);
		if (error !== undefined) {
			Dialog.errorMessage(error);
		}
	},

	async importSBML() {
		const filePath = await TAURI.dialog.open({
			multiple: false,
			directory: false,
			filters: [{
				name: 'SBML file',
				extensions: ['sbml', 'xml']
			}],
			title: 'Import .SBML'
		});
		if (!filePath || filePath.length === 0) {
			return;
		}

		ModelEditor.setModelFilePath(filePath)

		const sbmlFileContent = await TAURI.fs.readTextFile(filePath);

		ModelEndpoints.sbmlToAeon(sbmlFileContent)
			.then((model) => {
				LiveModel.handleAeonModelImport(model);
			})
			.catch((errorMessage) => {
				Dialog.errorMessage(errorMessage);
			})
	},

	startAnalysis(aeonString) {
		if (aeonString === undefined) {
			Dialog.errorMessage("Empty model.")
			return undefined;
		}
		Windows.openComputationWindow(aeonString)
	},

	// Add a listener to each button to display hint texts when hovered.
	// For toggle buttons, add functions that enable actual toggling of the state value.
	_initEdgeMenu(menu) {
		// make hint work
		let hint = menu.getElementsByClassName("hint")[0];
		let buttons = menu.getElementsByClassName("button");
		for (var i = 0; i < buttons.length; i++) {
			let button = buttons[i];
			button.addEventListener("mouseenter", (e) => {
				hint.textContent = button.alt;
				hint.classList.remove("invisible");
			});
			button.addEventListener("mouseleave", (e) => {
				hint.classList.add("invisible");
			});
		}
		// Make observability button react to regulation state:
		let observability = document.getElementById("edge-menu-observability");
		observability.updateState = function(data) {
			let state = "off";
			if (data.observable) state = "on";
			if (state != observability.getAttribute("state")) {
				observability.setAttribute("state", state);
				observability.alt = observability.getAttribute("alt-"+state);
				observability.src = observability.getAttribute("src-"+state);
				// if the hint is visible, it must be showing alt of this button (because the value just changed)
				hint.textContent = observability.alt;
			}			
		};
		observability.addEventListener("click", (e) => {
			let selected = CytoscapeEditor.getSelectedRegulationPair();
			if (selected !== undefined) {
				LiveModel.toggleObservability(selected.regulator, selected.target);				
			}
		});
		menu.observabilityButton = observability;
		let monotonicity = document.getElementById("edge-menu-monotonicity");
		monotonicity.updateState = function(data) {		
			if (monotonicity.getAttribute("state") != data.monotonicity) {
				monotonicity.alt = monotonicity.getAttribute("alt-"+data.monotonicity);
				monotonicity.src = monotonicity.getAttribute("src-"+data.monotonicity);
				monotonicity.setAttribute("state", data.monotonicity);
				// if the hint is visible, it must be showing alt of this button (because the value just changed)
				hint.textContent = monotonicity.alt;
			}				
		};
		monotonicity.addEventListener("click", (e) => {
			let selected = CytoscapeEditor.getSelectedRegulationPair();
			if (selected !== undefined) {
				LiveModel.toggleMonotonicity(selected.regulator, selected.target);
			}
		});
		menu.monotonicityButton = monotonicity;
		let removeButton = document.getElementById("edge-menu-remove");
		removeButton.addEventListener("click", (e) => {
			let selected = CytoscapeEditor.getSelectedRegulationPair();
			if (selected !== undefined) {
				LiveModel.removeRegulation(selected.regulator, selected.target);
			}
		});
	},

	// Add a listener to each button which displays its alt as hint text when hovered
	// and make the buttons actually clickable with actions.
	_initNodeMenu: function(menu) {
		// make hint work
		let hint = menu.getElementsByClassName("hint")[0];
		let buttons = menu.getElementsByClassName("button");		
		for (var i = 0; i < buttons.length; i++) {
			let button = buttons[i];
			button.addEventListener("mouseenter", (e) => {			
				hint.textContent = button.alt;
				hint.classList.remove("invisible");
			});
			button.addEventListener("mouseleave", (e) => {
				hint.classList.add("invisible");
			});
		}
		// Remove node button
		let removeButton = document.getElementById("node-menu-remove");
		removeButton.addEventListener("click", (e) => {
			let selectedNodeId = CytoscapeEditor.getSelectedNodeId();
			if (selectedNodeId !== undefined) {
				LiveModel.removeVariable(selectedNodeId);
			}
		});
		// Edit node name button
		let editNameButton = document.getElementById("node-menu-edit-name");
		editNameButton.addEventListener("click", (e) => {
			let selectedNodeId = CytoscapeEditor.getSelectedNodeId();
			if (selectedNodeId !== undefined) {
				ModelEditor.focusNameInput(selectedNodeId);
			}
		});
		// Edit function button
		let editFunctionButton = document.getElementById("node-menu-edit-function");
		editFunctionButton.addEventListener("click", (e) => {
			let selectedNodeId = CytoscapeEditor.getSelectedNodeId();
			if (selectedNodeId !== undefined) {
				ModelEditor.focusFunctionInput(selectedNodeId);
			}
		})
	},

	// Add a hover listener to all side menu items to show hint when needed.
	// Add a click listener that will toggle the appropriate tab for each button.
	_initSideMenu: function(menu) {
		let groups = menu.getElementsByClassName("button-group");
		for (var i = 0; i < groups.length; i++) {
			let group = groups[i];			
			let button = group.getElementsByClassName("button")[0];
			let hint = group.getElementsByClassName("hint")[0];
			let tabId = button.getAttribute("tab-id");
			// Show hint popup on mouse enter when button is not selected.
			button.addEventListener("mouseenter", (e) => {
				let selected = button.classList.contains("selected");
				if (!selected) {
					group.style.width = "272px";
					hint.classList.remove("invisible");
				}				
			});
			// Hide hint popup on mouse leave
			button.addEventListener("mouseleave", (e) => {
				group.style.width = "72px";				
				hint.classList.add("invisible");			
			});
			// On click, if selected, close content. If not selected, switch to this tab.
			button.addEventListener("click", (e) => {
				let selected = button.classList.contains("selected");
				if (selected) {
					UI.closeContent();
				} else {
					UI.ensureContentTabOpen(tabId);
					// Also, hide the hint popup
					group.style.width = "72px";
					hint.classList.add("invisible");
				}				
			});
		}
	},
	
}
