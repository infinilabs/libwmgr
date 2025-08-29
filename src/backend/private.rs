//! Private macOS APIs.

use bitflags::bitflags;
use objc2_application_services::AXError;
use objc2_application_services::AXUIElement;
use objc2_core_foundation::CFArray;
use objc2_core_graphics::CGWindowID;
use std::ffi::c_int;

pub(crate) type CGSConnectionID = u32;
pub(crate) type CGSSpaceID = c_int;

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct CGSSpaceMask: c_int {
        const INCLUDE_CURRENT = 1 << 0;
        const INCLUDE_OTHERS  = 1 << 1;

        const INCLUDE_USER    = 1 << 2;
        const INCLUDE_OS      = 1 << 3;

        const VISIBLE         = 1 << 16;

        const CURRENT_SPACES = Self::INCLUDE_USER.bits() | Self::INCLUDE_CURRENT.bits();
        const OTHER_SPACES = Self::INCLUDE_USER.bits() | Self::INCLUDE_OTHERS.bits();
        const ALL_SPACES =
            Self::INCLUDE_USER.bits() | Self::INCLUDE_OTHERS.bits() | Self::INCLUDE_CURRENT.bits();

        const ALL_VISIBLE_SPACES = Self::ALL_SPACES.bits() | Self::VISIBLE.bits();

        const CURRENT_OS_SPACES = Self::INCLUDE_OS.bits() | Self::INCLUDE_CURRENT.bits();
        const OTHER_OS_SPACES = Self::INCLUDE_OS.bits() | Self::INCLUDE_OTHERS.bits();
        const ALL_OS_SPACES =
            Self::INCLUDE_OS.bits() | Self::INCLUDE_OTHERS.bits() | Self::INCLUDE_CURRENT.bits();
    }
}

unsafe extern "C" {
    /// Extract `window_id` from an AXUIElement.
    pub fn _AXUIElementGetWindow(elem: *mut AXUIElement, window_id: *mut CGWindowID) -> AXError;

    /// Move the windows specified by `windows_ids` to the target workspace.
    pub fn CGSMoveWindowsToManagedSpace(
        connection: CGSConnectionID,
        window_ids: *const CFArray,
        target_space_id: CGSSpaceID,
    );

    pub fn CGSRemoveWindowsFromSpaces(
        connection: CGSConnectionID,
        window_ids: *const CFArray,
        spaces: *const CFArray,
    );
    pub fn CGSAddWindowsToSpaces(
        connection: CGSConnectionID,
        window_ids: *const CFArray,
        spaces: *const CFArray,
    );

    /// Connect to the WindowServer and get a connection descriptor.
    pub fn CGSMainConnectionID() -> CGSConnectionID;

    /// Return IDs of the available workspaces.
    pub fn CGSCopySpaces(cid: CGSConnectionID, mask: CGSSpaceMask) -> *const CFArray;

    /// It returns a CFArray of dictionaries. Each dictionary contains information
    /// about a display, including a list of all the spaces (CGSSpaceID) on that display.
    pub fn CGSCopyManagedDisplaySpaces(cid: CGSConnectionID) -> *mut CFArray;

    /// Gets the ID of the space currently visible to the user.
    pub fn CGSGetActiveSpace(cid: CGSConnectionID) -> CGSSpaceID;
}

#[cfg(test)]
mod tests {
    use std::{ffi::c_void, ops::Deref};

    use super::*;

    #[test]
    fn get_active_workspace() {
        unsafe {
            let conn = CGSMainConnectionID();
            let id = CGSGetActiveSpace(conn);
            println!("{}", id);
        }
    }

    #[test]
    fn list_all_workspaces() {
        use objc2_core_foundation::CFNumber;
        use objc2_core_foundation::CFRetained;
        use std::ptr::NonNull;

        unsafe {
            let conn = CGSMainConnectionID();
            let space_ids_raw = CGSCopySpaces(conn, CGSSpaceMask::ALL_SPACES);
            let space_ids: CFRetained<CFArray> =
                CFRetained::from_raw(NonNull::new(space_ids_raw.cast_mut()).unwrap());

            for idx in 0..space_ids.count() {
                let ptr = &*space_ids.value_at_index(idx).cast::<CFNumber>();
                let id: CGSSpaceID = ptr.as_i32().unwrap();
                println!("{}", id);
            }
        }
    }

    #[test]
    fn list_workspaces_grouped_by_display() {}
}
