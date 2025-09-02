#![cfg(target_os = "macos")]

pub mod actions;
mod backend;
pub mod error;

use crate::backend::get_frontmost_window_origin;
use crate::backend::get_next_workspace_logical_id;
use crate::backend::get_previous_workspace_logical_id;
use crate::backend::list_visible_frame_of_all_screens;
use crate::backend::move_frontmost_window;
use crate::backend::move_frontmost_window_to_workspace;
use crate::backend::set_frontmost_window_frame;
use actions::Action;
use backend::get_active_screen_visible_frame;
use backend::get_frontmost_window_size;
use backend::toggle_fullscreen;
use error::Error;
use objc2_core_foundation::{CGPoint, CGRect, CGSize};

/// Perform this action to the focused window.
///
/// NOTE: this function should be called in the main thread, or it will error out.
pub fn apply_to_focused_window(action: Action) -> Result<(), Error> {
    let visible_frame = get_active_screen_visible_frame()?;

    match action {
        Action::TopHalf => {
            let origin = CGPoint {
                x: visible_frame.origin.x,
                y: visible_frame.origin.y,
            };
            let size = CGSize {
                width: visible_frame.size.width,
                height: visible_frame.size.height / 2.0,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::BottomHalf => {
            let origin = CGPoint {
                x: visible_frame.origin.x,
                y: visible_frame.origin.y + visible_frame.size.height / 2.0,
            };
            let size = CGSize {
                width: visible_frame.size.width,
                height: visible_frame.size.height / 2.0,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::LeftHalf => {
            let origin = CGPoint {
                x: visible_frame.origin.x,
                y: visible_frame.origin.y,
            };
            let size = CGSize {
                width: visible_frame.size.width / 2.0,
                height: visible_frame.size.height,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::RightHalf => {
            let origin = CGPoint {
                x: visible_frame.origin.x + visible_frame.size.width / 2.0,
                y: visible_frame.origin.y,
            };
            let size = CGSize {
                width: visible_frame.size.width / 2.0,
                height: visible_frame.size.height,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::CenterHalf => {
            let origin = CGPoint {
                x: visible_frame.origin.x + visible_frame.size.width / 4.0,
                y: visible_frame.origin.y,
            };
            let size = CGSize {
                width: visible_frame.size.width / 2.0,
                height: visible_frame.size.height,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::TopLeftQuarter => {
            let origin = visible_frame.origin;
            let size = CGSize {
                width: visible_frame.size.width / 2.0,
                height: visible_frame.size.height / 2.0,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::TopRightQuarter => {
            let origin = CGPoint {
                x: visible_frame.origin.x + visible_frame.size.width / 2.0,
                y: visible_frame.origin.y,
            };
            let size = CGSize {
                width: visible_frame.size.width / 2.0,
                height: visible_frame.size.height / 2.0,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::BottomLeftQuarter => {
            let origin = CGPoint {
                x: visible_frame.origin.x,
                y: visible_frame.origin.y + visible_frame.size.height / 2.0,
            };
            let size = CGSize {
                width: visible_frame.size.width / 2.0,
                height: visible_frame.size.height / 2.0,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::BottomRightQuarter => {
            let origin = CGPoint {
                x: visible_frame.origin.x + visible_frame.size.width / 2.0,
                y: visible_frame.origin.y + visible_frame.size.height / 2.0,
            };
            let size = CGSize {
                width: visible_frame.size.width / 2.0,
                height: visible_frame.size.height / 2.0,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::TopLeftSixth => {
            let origin = visible_frame.origin;
            let size = CGSize {
                width: visible_frame.size.width / 3.0,
                height: visible_frame.size.height / 2.0,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::TopCenterSixth => {
            let origin = CGPoint {
                x: visible_frame.origin.x + visible_frame.size.width / 3.0,
                y: visible_frame.origin.y,
            };
            let size = CGSize {
                width: visible_frame.size.width / 3.0,
                height: visible_frame.size.height / 2.0,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::TopRightSixth => {
            let origin = CGPoint {
                x: visible_frame.origin.x + visible_frame.size.width * 2.0 / 3.0,
                y: visible_frame.origin.y,
            };
            let size = CGSize {
                width: visible_frame.size.width / 3.0,
                height: visible_frame.size.height / 2.0,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::BottomLeftSixth => {
            let origin = CGPoint {
                x: visible_frame.origin.x,
                y: visible_frame.origin.y + visible_frame.size.height / 2.0,
            };
            let size = CGSize {
                width: visible_frame.size.width / 3.0,
                height: visible_frame.size.height / 2.0,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::BottomCenterSixth => {
            let origin = CGPoint {
                x: visible_frame.origin.x + visible_frame.size.width / 3.0,
                y: visible_frame.origin.y + visible_frame.size.height / 2.0,
            };
            let size = CGSize {
                width: visible_frame.size.width / 3.0,
                height: visible_frame.size.height / 2.0,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::BottomRightSixth => {
            let origin = CGPoint {
                x: visible_frame.origin.x + visible_frame.size.width * 2.0 / 3.0,
                y: visible_frame.origin.y + visible_frame.size.height / 2.0,
            };
            let size = CGSize {
                width: visible_frame.size.width / 3.0,
                height: visible_frame.size.height / 2.0,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::TopThird => {
            let origin = visible_frame.origin;
            let size = CGSize {
                width: visible_frame.size.width,
                height: visible_frame.size.height / 3.0,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::MiddleThird => {
            let origin = CGPoint {
                x: visible_frame.origin.x,
                y: visible_frame.origin.y + visible_frame.size.height / 3.0,
            };
            let size = CGSize {
                width: visible_frame.size.width,
                height: visible_frame.size.height / 3.0,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::BottomThird => {
            let origin = CGPoint {
                x: visible_frame.origin.x,
                y: visible_frame.origin.y + visible_frame.size.height * 2.0 / 3.0,
            };
            let size = CGSize {
                width: visible_frame.size.width,
                height: visible_frame.size.height / 3.0,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::Center => {
            let window_size = get_frontmost_window_size()?;
            let origin = CGPoint {
                x: visible_frame.origin.x + (visible_frame.size.width - window_size.width) / 2.0,
                y: visible_frame.origin.y + (visible_frame.size.height - window_size.height) / 2.0,
            };
            move_frontmost_window(origin.x, origin.y)
        }
        Action::FirstFourth => {
            let origin = visible_frame.origin;
            let size = CGSize {
                width: visible_frame.size.width / 4.0,
                height: visible_frame.size.height,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::SecondFourth => {
            let origin = CGPoint {
                x: visible_frame.origin.x + visible_frame.size.width / 4.0,
                y: visible_frame.origin.y,
            };
            let size = CGSize {
                width: visible_frame.size.width / 4.0,
                height: visible_frame.size.height,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::ThirdFourth => {
            let origin = CGPoint {
                x: visible_frame.origin.x + visible_frame.size.width * 2.0 / 4.0,
                y: visible_frame.origin.y,
            };
            let size = CGSize {
                width: visible_frame.size.width / 4.0,
                height: visible_frame.size.height,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::LastFourth => {
            let origin = CGPoint {
                x: visible_frame.origin.x + visible_frame.size.width * 3.0 / 4.0,
                y: visible_frame.origin.y,
            };
            let size = CGSize {
                width: visible_frame.size.width / 4.0,
                height: visible_frame.size.height,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::FirstThird => {
            let origin = CGPoint {
                x: visible_frame.origin.x,
                y: visible_frame.origin.y,
            };
            let size = CGSize {
                width: visible_frame.size.width / 3.0,
                height: visible_frame.size.height,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::CenterThird => {
            let origin = CGPoint {
                x: visible_frame.origin.x + visible_frame.size.width / 3.0,
                y: visible_frame.origin.y,
            };
            let size = CGSize {
                width: visible_frame.size.width / 3.0,
                height: visible_frame.size.height,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::LastThird => {
            let origin = CGPoint {
                x: visible_frame.origin.x + visible_frame.size.width * 2.0 / 3.0,
                y: visible_frame.origin.y,
            };
            let size = CGSize {
                width: visible_frame.size.width / 3.0,
                height: visible_frame.size.height,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::FirstTwoThirds => {
            let origin = CGPoint {
                x: visible_frame.origin.x,
                y: visible_frame.origin.y,
            };
            let size = CGSize {
                width: visible_frame.size.width * 2.0 / 3.0,
                height: visible_frame.size.height,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::CenterTwoThirds => {
            let origin = CGPoint {
                x: visible_frame.origin.x + visible_frame.size.width / 6.0,
                y: visible_frame.origin.y,
            };
            let size = CGSize {
                width: visible_frame.size.width * 2.0 / 3.0,
                height: visible_frame.size.height,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::LastTwoThirds => {
            let origin = CGPoint {
                x: visible_frame.origin.x + visible_frame.size.width / 3.0,
                y: visible_frame.origin.y,
            };
            let size = CGSize {
                width: visible_frame.size.width * 2.0 / 3.0,
                height: visible_frame.size.height,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::FirstThreeFourths => {
            let origin = CGPoint {
                x: visible_frame.origin.x,
                y: visible_frame.origin.y,
            };
            let size = CGSize {
                width: visible_frame.size.width * 3.0 / 4.0,
                height: visible_frame.size.height,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::CenterThreeFourths => {
            let origin = CGPoint {
                x: visible_frame.origin.x + visible_frame.size.width / 8.0,
                y: visible_frame.origin.y,
            };
            let size = CGSize {
                width: visible_frame.size.width * 3.0 / 4.0,
                height: visible_frame.size.height,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::LastThreeFourths => {
            let origin = CGPoint {
                x: visible_frame.origin.x + visible_frame.size.width / 4.0,
                y: visible_frame.origin.y,
            };
            let size = CGSize {
                width: visible_frame.size.width * 3.0 / 4.0,
                height: visible_frame.size.height,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::TopThreeFourths => {
            let origin = CGPoint {
                x: visible_frame.origin.x,
                y: visible_frame.origin.y,
            };
            let size = CGSize {
                width: visible_frame.size.width,
                height: visible_frame.size.height * 3.0 / 4.0,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::BottomThreeFourths => {
            let origin = CGPoint {
                x: visible_frame.origin.x,
                y: visible_frame.origin.y + visible_frame.size.height / 4.0,
            };
            let size = CGSize {
                width: visible_frame.size.width,
                height: visible_frame.size.height * 3.0 / 4.0,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::TopTwoThirds => {
            let origin = CGPoint {
                x: visible_frame.origin.x,
                y: visible_frame.origin.y,
            };
            let size = CGSize {
                width: visible_frame.size.width,
                height: visible_frame.size.height * 2.0 / 3.0,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::BottomTwoThirds => {
            let origin = CGPoint {
                x: visible_frame.origin.x,
                y: visible_frame.origin.y + visible_frame.size.height / 3.0,
            };
            let size = CGSize {
                width: visible_frame.size.width,
                height: visible_frame.size.height * 2.0 / 3.0,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }

        Action::TopCenterTwoThirds => {
            let origin = CGPoint {
                x: visible_frame.origin.x + visible_frame.size.width / 6.0,
                y: visible_frame.origin.y,
            };
            let size = CGSize {
                width: visible_frame.size.width * 2.0 / 3.0,
                height: visible_frame.size.height * 2.0 / 3.0,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::TopFirstFourth => {
            let origin = CGPoint {
                x: visible_frame.origin.x,
                y: visible_frame.origin.y,
            };
            let size = CGSize {
                width: visible_frame.size.width,
                height: visible_frame.size.height / 4.0,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::TopSecondFourth => {
            let origin = CGPoint {
                x: visible_frame.origin.x,
                y: visible_frame.origin.y + visible_frame.size.height / 4.0,
            };
            let size = CGSize {
                width: visible_frame.size.width,
                height: visible_frame.size.height / 4.0,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::TopThirdFourth => {
            let origin = CGPoint {
                x: visible_frame.origin.x,
                y: visible_frame.origin.y + visible_frame.size.height * 2.0 / 4.0,
            };
            let size = CGSize {
                width: visible_frame.size.width,
                height: visible_frame.size.height / 4.0,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::TopLastFourth => {
            let origin = CGPoint {
                x: visible_frame.origin.x,
                y: visible_frame.origin.y + visible_frame.size.height * 3.0 / 4.0,
            };
            let size = CGSize {
                width: visible_frame.size.width,
                height: visible_frame.size.height / 4.0,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::MakeLarger => {
            let window_origin = get_frontmost_window_origin()?;
            let window_size = get_frontmost_window_size()?;
            let delta_width = 20_f64;
            let delta_height = window_size.height / window_size.width * delta_width;
            let delta_origin_x = delta_width / 2.0;
            let delta_origin_y = delta_height / 2.0;

            let new_width = {
                let possible_value = window_size.width + delta_width;
                if possible_value > visible_frame.size.width {
                    visible_frame.size.width
                } else {
                    possible_value
                }
            };
            let new_height = {
                let possible_value = window_size.height + delta_height;
                if possible_value > visible_frame.size.height {
                    visible_frame.size.height
                } else {
                    possible_value
                }
            };

            let new_origin_x = {
                let possible_value = window_origin.x - delta_origin_x;
                if possible_value < visible_frame.origin.x {
                    visible_frame.origin.x
                } else {
                    possible_value
                }
            };
            let new_origin_y = {
                let possible_value = window_origin.y - delta_origin_y;
                if possible_value < visible_frame.origin.y {
                    visible_frame.origin.y
                } else {
                    possible_value
                }
            };

            let origin = CGPoint {
                x: new_origin_x,
                y: new_origin_y,
            };
            let size = CGSize {
                width: new_width,
                height: new_height,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::MakeSmaller => {
            let window_origin = get_frontmost_window_origin()?;
            let window_size = get_frontmost_window_size()?;

            let delta_width = 20_f64;
            let delta_height = window_size.height / window_size.width * delta_width;

            let delta_origin_x = delta_width / 2.0;
            let delta_origin_y = delta_height / 2.0;

            let origin = CGPoint {
                x: window_origin.x + delta_origin_x,
                y: window_origin.y + delta_origin_y,
            };
            let size = CGSize {
                width: window_size.width - delta_width,
                height: window_size.height - delta_height,
            };
            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::AlmostMaximize => {
            let new_size = CGSize {
                width: visible_frame.size.width * 0.8,
                height: visible_frame.size.height * 0.8,
            };
            let new_origin = CGPoint {
                x: visible_frame.origin.x + (visible_frame.size.width * 0.1),
                y: visible_frame.origin.y + (visible_frame.size.height * 0.1),
            };
            let new_frame = CGRect {
                origin: new_origin,
                size: new_size,
            };
            set_frontmost_window_frame(new_frame)
        }
        Action::Maximize => {
            let new_frame = CGRect {
                origin: visible_frame.origin,
                size: visible_frame.size,
            };
            set_frontmost_window_frame(new_frame)
        }
        Action::MaximizeWidth => {
            let window_origin = get_frontmost_window_origin()?;
            let window_size = get_frontmost_window_size()?;
            let origin = CGPoint {
                x: visible_frame.origin.x,
                y: window_origin.y,
            };
            let size = CGSize {
                width: visible_frame.size.width,
                height: window_size.height,
            };

            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::MaximizeHeight => {
            let window_origin = get_frontmost_window_origin()?;
            let window_size = get_frontmost_window_size()?;
            let origin = CGPoint {
                x: window_origin.x,
                y: visible_frame.origin.y,
            };
            let size = CGSize {
                width: window_size.width,
                height: visible_frame.size.height,
            };

            let new_frame = CGRect { origin, size };
            set_frontmost_window_frame(new_frame)
        }
        Action::MoveUp => {
            let window_origin = get_frontmost_window_origin()?;
            let new_y = (window_origin.y - 10.0).max(visible_frame.origin.y);
            move_frontmost_window(window_origin.x, new_y)
        }
        Action::MoveDown => {
            let window_origin = get_frontmost_window_origin()?;
            let window_size = get_frontmost_window_size()?;
            let new_y = (window_origin.y + 10.0)
                .min(visible_frame.origin.y + visible_frame.size.height - window_size.height);
            move_frontmost_window(window_origin.x, new_y)
        }
        Action::MoveLeft => {
            let window_origin = get_frontmost_window_origin()?;
            let new_x = (window_origin.x - 10.0).max(visible_frame.origin.x);
            move_frontmost_window(new_x, window_origin.y)
        }
        Action::MoveRight => {
            let window_origin = get_frontmost_window_origin()?;
            let window_size = get_frontmost_window_size()?;
            let new_x = (window_origin.x + 10.0)
                .min(visible_frame.origin.x + visible_frame.size.width - window_size.width);
            move_frontmost_window(new_x, window_origin.y)
        }
        Action::NextDesktop => {
            let Some(next_workspace_logical_id) = get_next_workspace_logical_id() else {
                // nothing to do
                return Ok(());
            };

            move_frontmost_window_to_workspace(next_workspace_logical_id)
        }
        Action::PreviousDesktop => {
            let Some(previous_workspace_logical_id) = get_previous_workspace_logical_id() else {
                // nothing to do
                return Ok(());
            };

            // Now let's switch the workspace
            move_frontmost_window_to_workspace(previous_workspace_logical_id)
        }
        Action::NextDisplay => {
            const TOO_MANY_MONITORS: &str = "I don't think you can have so many monitors";

            let frames = list_visible_frame_of_all_screens()?;
            let n_frames = frames.len();
            if n_frames == 0 {
                return Err(Error::NoDisplay);
            }
            if n_frames == 1 {
                return Ok(());
            }

            let index = frames
                .iter()
                .position(|fr| fr == &visible_frame)
                .expect("active screen should be in the list");
            let new_index: usize = {
                let index_i32: i32 = index.try_into().expect(TOO_MANY_MONITORS);
                let index_i32_plus_one = index_i32.checked_add(1).expect(TOO_MANY_MONITORS);
                let final_value = index_i32_plus_one % n_frames as i32;

                final_value
                    .try_into()
                    .expect("final value should be positive")
            };

            let new_frame = frames[new_index];

            set_frontmost_window_frame(new_frame)
        }
        Action::PreviousDisplay => {
            const TOO_MANY_MONITORS: &str = "I don't think you can have so many monitors";

            let frames = list_visible_frame_of_all_screens()?;
            let n_frames = frames.len();
            if n_frames == 0 {
                return Err(Error::NoDisplay);
            }
            if n_frames == 1 {
                return Ok(());
            }
            let index = frames
                .iter()
                .position(|fr| fr == &visible_frame)
                .expect("active screen should be in the list");
            let new_index: usize = {
                let index_i32: i32 = index.try_into().expect(TOO_MANY_MONITORS);
                let index_i32_minus_one = index_i32 - 1;
                let n_frames_i32: i32 = n_frames.try_into().expect(TOO_MANY_MONITORS);
                let final_value = (index_i32_minus_one + n_frames_i32) % n_frames_i32;

                final_value
                    .try_into()
                    .expect("final value should be positive")
            };

            let new_frame = frames[new_index];

            set_frontmost_window_frame(new_frame)
        }
        Action::Restore => {
            todo!()
        }
        Action::ToggleFullscreen => toggle_fullscreen(),
    }
}
