use std::cell::RefCell;

thread_local! {
    static LAST_PLUGIN_ERROR: RefCell<Option<String>> = const { RefCell::new(None) };
}

/// Record a human-readable error from a plugin hook (same OS thread as the hook).
pub fn set_last_plugin_error(msg: impl Into<String>) {
    LAST_PLUGIN_ERROR.with(|slot| *slot.borrow_mut() = Some(msg.into()));
}

/// Take and clear the last plugin error for the current thread.
pub fn take_last_plugin_error() -> Option<String> {
    LAST_PLUGIN_ERROR.with(|slot| slot.borrow_mut().take())
}
