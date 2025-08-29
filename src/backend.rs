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

use crate::error::Error;

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
