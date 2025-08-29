use objc2_application_services::AXError;

#[derive(Debug)]
pub enum Error {
    CannotFindFocusWindow,
    AXError(AXError),
    NotInMainThread,
    NoDisplay,
}
