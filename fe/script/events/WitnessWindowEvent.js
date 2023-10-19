// Listen for 'open-editor-tab' event to open tab with model info
TAURI.event.listen('open-editor-tab', (event) => {
    UI.ensureContentTabOpen(ContentTabs.modelEditor);
});