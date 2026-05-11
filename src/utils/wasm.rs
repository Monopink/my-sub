use wasm_bindgen::prelude::*;

/// Set up better panic hook for WebAssembly that shows proper stack traces
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if a panic occurs.
    //#[cfg(feature = "console_error_panic_hook")]
    //console_error_panic_hook::set_once();

    // For unconditional initialization without the feature flag:
    // This is useful in development or when troubleshooting specific issues
    // Remove this in production if you don't want the overhead
    //#[cfg(not(feature = "console_error_panic_hook"))]
    {
        use console_error_panic_hook::hook;
        use std::panic;
        static HOOK_SET: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

        if !HOOK_SET.swap(true, std::sync::atomic::Ordering::SeqCst) {
            let prev_hook = panic::take_hook();
            panic::set_hook(Box::new(move |panic_info| {
                // Call the hook from console_error_panic_hook first
                hook(panic_info);
                // Then call the previous hook
                prev_hook(panic_info);
            }));

            log::debug!("WebAssembly panic hook with stack traces enabled");
        }
    }
}

/// Initialize WebAssembly panic hook only (not logging)
/// This function is named differently to avoid name collision with the one in api/init.rs
#[wasm_bindgen]
pub fn init_panic_hook() {
    // Set up panic hook for better error messages
    set_panic_hook();

    log::info!("WebAssembly panic hook initialized for better stack traces");
}
