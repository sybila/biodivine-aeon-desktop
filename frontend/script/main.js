hasLocalStorage = false;

async function init() {

	// Set program version
	let version = await TAURI.invoke('get_version', {})
	document.title = document.title + " (" + version + ")";
	document.getElementById("version").innerHTML = "v" + version


	try {
		localStorage.setItem('testing', '1');
		hasLocalStorage = true;
		console.log("Local storage available.");
	} catch (e) {
		console.log("Local storage not available.");
	}
	
	UI.init();
	ModelEditor.init();
	CytoscapeEditor.init();

	// Emit when the window is fully initialized and ready
	TAURI.event.emit('ready', {});
}


/* This can be used to properly show placeholder for content editable stuff */
function fixEmptyEditable(e) {
	if (e.target.textContent.trim().length === 0) {
		e.target.textContent = "";		
	}
}

function ensurePlaceholder(el) {
	el.addEventListener("focusout", fixEmptyEditable);	
}

/*
	"Data types":
	id: Number
	regulation: {
		regulator: Id,
		target: Id,
		observable: bool,
		monotonicity: string from EdgeMonotonicity
	}
*/

hotkeys('e', function(event, handler) {	
	if (UI.isNodeMenuVisible()) {
		event.preventDefault();
		fireEvent(document.getElementById("node-menu-edit-name"), "click");
	}	
});

hotkeys('f', function(event, handler) {	
	if (UI.isNodeMenuVisible()) {
		event.preventDefault();
		fireEvent(document.getElementById("node-menu-edit-function"), "click");
	}	
});

hotkeys('backspace', function(event, handler) {	
	if (UI.isNodeMenuVisible()) {
		event.preventDefault();
		fireEvent(document.getElementById("node-menu-remove"), "click");
	}	
	if (UI.isEdgeMenuVisible()) {
		event.preventDefault();
		fireEvent(document.getElementById("edge-menu-remove"), "click");
	}
});

hotkeys('o', function(event, handler) {	
	if (UI.isEdgeMenuVisible()) {
		event.preventDefault();
		fireEvent(document.getElementById("edge-menu-observability"), "click");
	}	
});

hotkeys('m', function(event, handler) {	
	if (UI.isEdgeMenuVisible()) {
		event.preventDefault();
		fireEvent(document.getElementById("edge-menu-monotonicity"), "click");
	}	
});

hotkeys('n,+', function(event, handler) {	
	event.preventDefault();
	let id = LiveModel.addVariable();
	CytoscapeEditor.showNode(id);
});

hotkeys('h', { keyup: true }, function(event, handler) {
	if (event.type === 'keydown') {
		UI.setQuickHelpVisible(true);
	}
	if (event.type === 'keyup') {
		UI.setQuickHelpVisible(false);
	}	
});


// utility function to fire events on UI elements - we mainly need it to simulate clicks
function fireEvent(el, etype){
  if (el.fireEvent) {
    el.fireEvent('on' + etype);
  } else {
    var evObj = document.createEvent('Events');
    evObj.initEvent(etype, true, false);
    el.dispatchEvent(evObj);
  }
}
