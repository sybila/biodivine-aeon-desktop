let Session = {
    _windowSessionKey: undefined,

    createWindowSession(windowSessionKey) {
        this._windowSessionKey = windowSessionKey
        TAURI.invoke('add_window_session', { windowSessionKey: windowSessionKey })
    },

    destroyWindowSession() {
        return TAURI.invoke('remove_window_session', { windowSessionKey: this._windowSessionKey })
    },

    hasRunningComputation() {
        return TAURI.invoke('has_running_computation', {windowSessionKey: this._windowSessionKey})
    }
}
