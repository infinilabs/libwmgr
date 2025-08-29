use objc2_application_services::AXError;

#[derive(Debug)]
pub enum Error {
    /// Cannot find the focused window.
    CannotFindFocusWindow,
    /// Error code from the macOS Accessibility APIs.
    AXError(AXError),
    /// Function should be in called from the main thread, but it is not.
    NotInMainThread,
    /// No monitor detected.
    NoDisplay,
}
