let SessionCommands = {
    _sessionKey: undefined,

    createSession(sessionKey) {
        this._sessionKey = sessionKey
        TAURI.invoke('add_session', { sessionKey: sessionKey })
    },

    destroySession() {
        return TAURI.invoke('remove_session', { sessionKey: this._sessionKey })
    },

    hasRunningComputation() {
        return TAURI.invoke('has_running_computation', {sessionKey: this._sessionKey})
    }
}
