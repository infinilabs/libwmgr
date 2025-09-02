use objc2_application_services::AXError;
use objc2_core_graphics::CGError;

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
    /// Already the last desktop.
    AlreadyInLastDesktop,
    /// Already the first desktop.
    AlreadyInFirstDesktop,
    /// libwmgr can only handle 16 Workspaces at most.
    TooManyWorkspace,
    /// Error code from the macOS Core Graphics APIs.
    CGError(CGError),
}
