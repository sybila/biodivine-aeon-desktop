let Model = {

    // Read .aeon model string and return model name.
    // Return undefined if model name is not defined.
    getModelName(modelString) {
        let modelNameRegex = /^\s*#name:(.+)$/
        let lines = modelString.split("\n")
        let modelName = undefined

        for (let line of lines) {
            if (line.trim().length === 0) continue	// skip whitespace
            let match = line.match(modelNameRegex)
            if (match !== null) {
                modelName = match[1]
                break
            }
        }
        return modelName
    },

}