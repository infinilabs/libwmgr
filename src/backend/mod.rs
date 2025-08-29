mod private;

use std::ffi::c_void;
use std::ops::Deref;
use std::ptr::NonNull;

use objc2::MainThreadMarker;
use objc2_app_kit::NSScreen;
use objc2_app_kit::NSWorkspace;
use objc2_application_services::AXError;
use objc2_application_services::AXUIElement;
use objc2_application_services::AXValue;
use objc2_application_services::AXValueType;
use objc2_core_foundation::CFBoolean;
use objc2_core_foundation::CFRetained;
use objc2_core_foundation::CFString;
use objc2_core_foundation::CFType;
use objc2_core_foundation::CGPoint;
use objc2_core_foundation::CGRect;
use objc2_core_foundation::CGSize;
use objc2_core_foundation::Type;
use objc2_core_foundation::{CFArray, CFDictionary, CFNumber};
use objc2_core_graphics::CGWindowID;

use crate::backend::private::CGSConnectionID;
use crate::error::Error;

use private::CGSCopyManagedDisplaySpaces;
use private::CGSGetActiveSpace;
use private::CGSMainConnectionID;
use private::CGSSpaceID;

pub fn frame_contains_point(frame: &CGRect, point: &CGPoint) -> bool {
    let min = frame.min();
    let max = frame.max();

    // NOTE that when comparing with `max`, we use < rather than <=
    let x_in_range = point.x >= min.x && point.x < max.x;
    let y_in_range = point.y >= min.y && point.y < max.y;

    x_in_range && y_in_range
}

fn flip_frame_y(main_screen_height: f64, frame_height: f64, frame_unflipped_y: f64) -> f64 {
    main_screen_height - (frame_unflipped_y + frame_height)
}

fn get_frontmost_window() -> Result<CFRetained<AXUIElement>, Error> {
    let workspace = unsafe { NSWorkspace::sharedWorkspace() };
    let frontmost_app =
        unsafe { workspace.frontmostApplication() }.ok_or(Error::CannotFindFocusWindow)?;

    let pid = unsafe { frontmost_app.processIdentifier() };

    let app_element = unsafe { AXUIElement::new_application(pid) };

    let mut window_element: *const CFType = std::ptr::null();
    let ptr_to_window_element = NonNull::new(&mut window_element).unwrap();
    let focused_window_attr = CFString::from_static_str("AXFocusedWindow");

    let error =
        unsafe { app_element.copy_attribute_value(&focused_window_attr, ptr_to_window_element) };

    if error != AXError::Success {
        return Err(Error::AXError(error));
    }
    assert!(!window_element.is_null());

    let window_element: *mut AXUIElement = window_element.cast::<AXUIElement>().cast_mut();

    let window = unsafe { CFRetained::from_raw(NonNull::new(window_element).unwrap()) };

    Ok(window)
}

pub(crate) fn get_frontmost_window_id() -> Result<CGWindowID, Error> {
    let element = get_frontmost_window()?;
    let ptr: NonNull<AXUIElement> = CFRetained::as_ptr(&element);

    let mut window_id_buffer: CGWindowID = 0;
    let error =
        unsafe { private::_AXUIElementGetWindow(ptr.as_ptr(), &mut window_id_buffer as *mut _) };
    if error != AXError::Success {
        return Err(Error::AXError(error));
    }

    Ok(window_id_buffer)
}

fn workspace_ids_grouped_by_display() -> Vec<Vec<CGSSpaceID>> {
    unsafe {
        let mut ret = Vec::new();
        let conn = CGSMainConnectionID();

        let display_spaces_raw = CGSCopyManagedDisplaySpaces(conn);
        let display_spaces: CFRetained<CFArray> =
            CFRetained::from_raw(NonNull::new(display_spaces_raw).unwrap());

        let key_spaces: CFRetained<CFString> = CFString::from_static_str("Spaces");
        let key_spaces_ptr: NonNull<CFString> = CFRetained::as_ptr(&key_spaces);
        let key_id64: CFRetained<CFString> = CFString::from_static_str("id64");
        let key_id64_ptr: NonNull<CFString> = CFRetained::as_ptr(&key_id64);

        for i in 0..display_spaces.count() {
            let mut workspaces_of_this_display = Vec::new();

            let dict_ref = display_spaces.value_at_index(i);
            let dict: &CFDictionary = &*(dict_ref as *const CFDictionary);

            let mut ptr_to_value_buffer: *const c_void = std::ptr::null();
            let key_exists = dict.value_if_present(
                key_spaces_ptr.as_ptr().cast::<c_void>().cast_const(),
                &mut ptr_to_value_buffer as *mut _,
            );
            assert!(key_exists);
            assert!(!ptr_to_value_buffer.is_null());

            let spaces_raw: *const CFArray = ptr_to_value_buffer.cast::<CFArray>();

            let spaces = &*spaces_raw;

            for idx in 0..spaces.count() {
                let workspace_dictionary: &CFDictionary =
                    &*spaces.value_at_index(idx).cast::<CFDictionary>();

                let mut ptr_to_value_buffer: *const c_void = std::ptr::null();
                let key_exists = workspace_dictionary.value_if_present(
                    key_id64_ptr.as_ptr().cast::<c_void>().cast_const(),
                    &mut ptr_to_value_buffer as *mut _,
                );
                assert!(key_exists);
                assert!(!ptr_to_value_buffer.is_null());

                let ptr_workspace_id = ptr_to_value_buffer.cast::<CFNumber>();
                let workspace_id = (&*ptr_workspace_id).as_i32().unwrap();

                workspaces_of_this_display.push(workspace_id);
            }

            ret.push(workspaces_of_this_display);
        }

        ret
    }
}

fn move_window_to_workspace(
    window_server_connection: CGSConnectionID,
    window_id: CGWindowID,
    source_workspace_id: CGSSpaceID,
    target_workspace_id: CGSSpaceID,
) {
    println!("DBG:");

    let window_id: CFRetained<CFNumber> = CFNumber::new_i64(window_id as i64).retain();
    let window_ids = CFArray::from_retained_objects(&[window_id]);
    let window_ids_non_ptr = CFRetained::as_ptr(&window_ids);
    let window_ids_ptr: *const CFArray<CFNumber> = window_ids_non_ptr.as_ptr().cast_const();
    let window_ids_ptr_opaque_type: *const CFArray = window_ids_ptr.cast();

    let source_workspace_id: CFRetained<CFNumber> = CFNumber::new_i32(source_workspace_id).retain();
    let source_workspace_ids = CFArray::from_retained_objects(&[source_workspace_id]);
    let source_workspaces_ids_non_ptr = CFRetained::as_ptr(&source_workspace_ids);
    let source_workspaces_ids_ptr: *const CFArray<CFNumber> =
        source_workspaces_ids_non_ptr.as_ptr().cast_const();
    let source_workspaces_ids_ptr_opaque_type: *const CFArray = source_workspaces_ids_ptr.cast();

    let target_workspace_id: CFRetained<CFNumber> = CFNumber::new_i32(target_workspace_id).retain();
    let target_workspace_ids = CFArray::from_retained_objects(&[target_workspace_id]);
    let target_workspaces_ids_non_ptr = CFRetained::as_ptr(&target_workspace_ids);
    let target_workspaces_ids_ptr: *const CFArray<CFNumber> =
        target_workspaces_ids_non_ptr.as_ptr().cast_const();
    let target_workspaces_ids_ptr_opaque_type: *const CFArray = target_workspaces_ids_ptr.cast();

    unsafe {
        private::CGSRemoveWindowsFromSpaces(
            window_server_connection,
            window_ids_ptr_opaque_type,
            source_workspaces_ids_ptr_opaque_type,
        );
        private::CGSAddWindowsToSpaces(
            window_server_connection,
            window_ids_ptr_opaque_type,
            target_workspaces_ids_ptr_opaque_type,
        )
    }
}

pub(crate) fn next_desktop() -> Result<(), Error> {
    unsafe {
        let window_server_connection = CGSMainConnectionID();
        let frontmost_window_id = get_frontmost_window_id()?;
        let current_workspace_id = CGSGetActiveSpace(window_server_connection);

        for mut workspaces in workspace_ids_grouped_by_display() {
            // Sort the IDs in ascending order
            workspaces.sort();

            match workspaces.binary_search(&current_workspace_id) {
                Ok(index) => {
                    // Found it!

                    // If this is the last workspace, nothing to do!
                    if index + 1 == workspaces.len() {
                        return Err(Error::AlreadyInLastDesktop);
                    }

                    let new_workspace_id = workspaces[index + 1];
                    move_window_to_workspace(
                        window_server_connection,
                        frontmost_window_id,
                        current_workspace_id,
                        new_workspace_id,
                    );
                    return Ok(());
                }
                Err(_) => {
                    // The active workspace is not in this screen, check next one.
                    continue;
                }
            }
        }

        unreachable!()
    }
}

pub(crate) fn get_frontmost_window_origin() -> Result<CGPoint, Error> {
    let frontmost_window = get_frontmost_window()?;

    let mut position_value: *const CFType = std::ptr::null();
    let ptr_to_position_value = NonNull::new(&mut position_value).unwrap();
    let position_attr = CFString::from_static_str("AXPosition");
    let error =
        unsafe { frontmost_window.copy_attribute_value(&position_attr, ptr_to_position_value) };

    if error != AXError::Success {
        return Err(Error::AXError(error));
    }
    assert!(!position_value.is_null());

    let position: CFRetained<AXValue> =
        unsafe { CFRetained::from_raw(NonNull::new(position_value.cast_mut().cast()).unwrap()) };

    let mut position_cg_point = CGPoint::ZERO;
    let ptr_to_position_cg_point =
        NonNull::new((&mut position_cg_point as *mut CGPoint).cast()).unwrap();

    let result = unsafe { position.value(AXValueType::CGPoint, ptr_to_position_cg_point) };
    assert!(result, "type mismatched");

    Ok(position_cg_point)
}

pub(crate) fn get_frontmost_window_size() -> Result<CGSize, Error> {
    let frontmost_window = get_frontmost_window()?;

    let mut size_value: *const CFType = std::ptr::null();
    let ptr_to_size_value = NonNull::new(&mut size_value).unwrap();
    let size_attr = CFString::from_static_str("AXSize");
    let error = unsafe { frontmost_window.copy_attribute_value(&size_attr, ptr_to_size_value) };

    if error != AXError::Success {
        return Err(Error::AXError(error));
    }
    assert!(!size_value.is_null());

    let size: CFRetained<AXValue> =
        unsafe { CFRetained::from_raw(NonNull::new(size_value.cast_mut().cast()).unwrap()) };

    let mut size_cg_size = CGSize::ZERO;
    let ptr_to_size_cg_size = NonNull::new((&mut size_cg_size as *mut CGSize).cast()).unwrap();

    let result = unsafe { size.value(AXValueType::CGSize, ptr_to_size_cg_size) };
    assert!(result, "type mismatched");

    Ok(size_cg_size)
}

pub(crate) fn list_visible_frame_of_all_screens() -> Vec<CGRect> {
    let main_thread_marker = MainThreadMarker::new().expect("not in the main thread");
    let screens = NSScreen::screens(main_thread_marker).to_vec();

    if screens.is_empty() {
        return Vec::new();
    }

    let main_screen = screens.first().expect("screens is not empty");

    screens
        .iter()
        .map(|ns_screen| {
            let mut unflipped_frame = ns_screen.visibleFrame();
            let flipped_frame_origin_y = flip_frame_y(
                main_screen.frame().size.height,
                unflipped_frame.size.height,
                unflipped_frame.origin.y,
            );
            unflipped_frame.origin.y = flipped_frame_origin_y;

            unflipped_frame
        })
        .collect()
}

pub(crate) fn get_active_screen_visible_frame() -> Result<CGRect, Error> {
    let main_thread_marker = MainThreadMarker::new().ok_or(Error::NotInMainThread)?;
    let main_screen = NSScreen::mainScreen(main_thread_marker).ok_or(Error::NoDisplay)?;

    let frontmost_window_origin = get_frontmost_window_origin()?;
    let main_screen_height = main_screen.frame().size.height;

    // AppKit uses Unflipped Coordinate System, but Accessibility APIs use
    // Flipped Coordinate System, we need to flip the origin of these screens.
    for screen in NSScreen::screens(main_thread_marker).into_iter() {
        let is_main_screen = screen == main_screen;
        let frame = if is_main_screen {
            screen.frame()
        } else {
            // Need flip
            let mut frame = screen.frame();
            let unflipped_y = frame.origin.y;
            let flipped_y = flip_frame_y(main_screen_height, frame.size.height, unflipped_y);
            frame.origin.y = flipped_y;

            frame
        };
        let mut visible_frame = screen.visibleFrame();
        let flipped_y = flip_frame_y(
            main_screen_height,
            visible_frame.size.height,
            visible_frame.origin.y,
        );
        visible_frame.origin.y = flipped_y;

        if frame_contains_point(&frame, &frontmost_window_origin) {
            return Ok(visible_frame);
        }
    }

    todo!("FIXME: it is possible that this window's origin is not in any screen!")
}

pub fn move_window(x: f64, y: f64) -> Result<(), Error> {
    let frontmost_window = get_frontmost_window()?;

    let mut point = CGPoint::new(x, y);
    let ptr_to_point = NonNull::new((&mut point as *mut CGPoint).cast::<c_void>()).unwrap();
    let pos_value = unsafe { AXValue::new(AXValueType::CGPoint, ptr_to_point) }.unwrap();
    let pos_attr = CFString::from_static_str("AXPosition");

    let error = unsafe { frontmost_window.set_attribute_value(&pos_attr, pos_value.deref()) };
    if error != AXError::Success {
        return Err(Error::AXError(error));
    }

    Ok(())
}

pub fn set_window_frame(frame: CGRect) -> Result<(), Error> {
    let frontmost_window = get_frontmost_window()?;

    let mut point = frame.origin;
    let ptr_to_point = NonNull::new((&mut point as *mut CGPoint).cast::<c_void>()).unwrap();
    let pos_value = unsafe { AXValue::new(AXValueType::CGPoint, ptr_to_point) }.expect("TODO");
    let pos_attr = CFString::from_static_str("AXPosition");

    let error = unsafe { frontmost_window.set_attribute_value(&pos_attr, pos_value.deref()) };
    if error != AXError::Success {
        return Err(Error::AXError(error));
    }

    let mut size = frame.size;
    let ptr_to_size = NonNull::new((&mut size as *mut CGSize).cast::<c_void>()).unwrap();
    let size_value = unsafe { AXValue::new(AXValueType::CGSize, ptr_to_size) }.expect("TODO");
    let size_attr = CFString::from_static_str("AXSize");

    let error = unsafe { frontmost_window.set_attribute_value(&size_attr, size_value.deref()) };
    if error != AXError::Success {
        return Err(Error::AXError(error));
    }

    Ok(())
}

pub fn toggle_fullscreen() -> Result<(), Error> {
    let frontmost_window = get_frontmost_window()?;
    let fullscreen_attr = CFString::from_static_str("AXFullScreen");

    let mut current_value_ref: *const CFType = std::ptr::null();
    let error = unsafe {
        frontmost_window.copy_attribute_value(
            &fullscreen_attr,
            NonNull::new(&mut current_value_ref).unwrap(),
        )
    };

    // TODO: If the attribute doesn't exist, error won't be Success as well.
    // Before we handle that, we need to know the error case that will be
    // returned in that case.
    if error != AXError::Success {
        return Err(Error::AXError(error));
    }
    assert!(!current_value_ref.is_null());

    let current_value = unsafe {
        let retained_boolean: CFRetained<CFBoolean> = CFRetained::from_raw(
            NonNull::new(current_value_ref.cast::<CFBoolean>().cast_mut()).unwrap(),
        );
        retained_boolean.as_bool()
    };

    let new_value = !current_value;
    let new_value_ref: CFRetained<CFBoolean> = CFBoolean::new(new_value).retain();

    let error =
        unsafe { frontmost_window.set_attribute_value(&fullscreen_attr, new_value_ref.deref()) };

    if error != AXError::Success {
        return Err(Error::AXError(error));
    }

    Ok(())
}
