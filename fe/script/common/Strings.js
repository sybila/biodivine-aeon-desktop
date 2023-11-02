let Strings = {
    modelEmpty: "Cannot export an empty model.",
    modelWillBeOverwritten: "Would you like to overwrite your current model?",
    openNewWindow: "Editor is not empty. Do you want to open the model in a new window?",
    runningComputation: "Computation is still running. Do you want to close window?",
    waitingForTreeResult: "Waiting for result. Do you really want to close the window?",

    removeNodeCheck(name) {
        return "Dou you really want to remove '"+name+"'?";
    },

    invalidVariableName(name) {
        return "Cannot use '"+name+"' as variable name.";
    },

    invalidUpdateFunction(name) {
        return "Cannot set update function for '"+name+"'.";
    },
}
