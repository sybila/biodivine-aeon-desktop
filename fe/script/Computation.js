let Computation = {
    _windowSessionKey: undefined,
    _lastComputation: undefined,

    setWindowSessionKey(windowSessionKey) {
        this._windowSessionKey = windowSessionKey
    },

    getWindowSessionKey() {
        return this._windowSessionKey
    },

    setLastComputation(timestamp) {
        this._lastComputation = timestamp
    },

    setActiveComputation(timestamp) {
        if (this._lastComputation !== timestamp) {
            // if timestamp changed, switch to undefined.
            this._lastComputation = undefined;
        }
    },

    isActiveComputation() {
        return this._lastComputation !== undefined;
    },
}