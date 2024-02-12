//  Main onload function for computation window
async function init() {

	// Set program version
	let version = await TAURI.core.invoke("get_version", {});
	document.title = document.title + " (" + version + ")";
	document.getElementById("version").innerHTML = "v" + version;

	// Emit when the window is fully initialized and ready
	TAURI.event.emit("ready", {});
}