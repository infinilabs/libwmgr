mod backend;
mod commands;
pub mod error;

use crate::backend::get_frontmost_window_origin;
use crate::backend::list_visible_frame_of_all_screens;
use crate::backend::move_window;
use crate::backend::set_window_frame;
use backend::get_active_screen_visible_frame;
use backend::get_frontmost_window_size;
use backend::toggle_fullscreen;
use commands::Command;
use objc2_core_foundation::{CGPoint, CGRect, CGSize};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: wmgr <option>");
        std::process::exit(1);
    }

    let command_str = &args[1];
    let command = serde_plain::from_str::<Command>(command_str).unwrap_or_else(|error| {
        println!("Invalid command [{}], error [{}]", command_str, error);
        std::process::exit(1);
    });

    let opt_visible_frame = get_active_screen_visible_frame().unwrap();

    let Some(frame) = opt_visible_frame else {
        return;
    };

    match command {
        Command::TopHalf => {
            let origin = CGPoint {
                x: frame.origin.x,
                y: frame.origin.y,
            };
            let size = CGSize {
                width: frame.size.width,
                height: frame.size.height / 2.0,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::BottomHalf => {
            let origin = CGPoint {
                x: frame.origin.x,
                y: frame.origin.y + frame.size.height / 2.0,
            };
            let size = CGSize {
                width: frame.size.width,
                height: frame.size.height / 2.0,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::LeftHalf => {
            let origin = CGPoint {
                x: frame.origin.x,
                y: frame.origin.y,
            };
            let size = CGSize {
                width: frame.size.width / 2.0,
                height: frame.size.height,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::RightHalf => {
            let origin = CGPoint {
                x: frame.origin.x + frame.size.width / 2.0,
                y: frame.origin.y,
            };
            let size = CGSize {
                width: frame.size.width / 2.0,
                height: frame.size.height,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::CenterHalf => {
            let origin = CGPoint {
                x: frame.origin.x + frame.size.width / 4.0,
                y: frame.origin.y,
            };
            let size = CGSize {
                width: frame.size.width / 2.0,
                height: frame.size.height,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::TopLeftQuarter => {
            let origin = frame.origin;
            let size = CGSize {
                width: frame.size.width / 2.0,
                height: frame.size.height / 2.0,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::TopRightQuarter => {
            let origin = CGPoint {
                x: frame.origin.x + frame.size.width / 2.0,
                y: frame.origin.y,
            };
            let size = CGSize {
                width: frame.size.width / 2.0,
                height: frame.size.height / 2.0,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::BottomLeftQuarter => {
            let origin = CGPoint {
                x: frame.origin.x,
                y: frame.origin.y + frame.size.height / 2.0,
            };
            let size = CGSize {
                width: frame.size.width / 2.0,
                height: frame.size.height / 2.0,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::BottomRightQuarter => {
            let origin = CGPoint {
                x: frame.origin.x + frame.size.width / 2.0,
                y: frame.origin.y + frame.size.height / 2.0,
            };
            let size = CGSize {
                width: frame.size.width / 2.0,
                height: frame.size.height / 2.0,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::TopLeftSixth => {
            let origin = frame.origin;
            let size = CGSize {
                width: frame.size.width / 3.0,
                height: frame.size.height / 2.0,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::TopCenterSixth => {
            let origin = CGPoint {
                x: frame.origin.x + frame.size.width / 3.0,
                y: frame.origin.y,
            };
            let size = CGSize {
                width: frame.size.width / 3.0,
                height: frame.size.height / 2.0,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::TopRightSixth => {
            let origin = CGPoint {
                x: frame.origin.x + frame.size.width * 2.0 / 3.0,
                y: frame.origin.y,
            };
            let size = CGSize {
                width: frame.size.width / 3.0,
                height: frame.size.height / 2.0,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::BottomLeftSixth => {
            let origin = CGPoint {
                x: frame.origin.x,
                y: frame.origin.y + frame.size.height / 2.0,
            };
            let size = CGSize {
                width: frame.size.width / 3.0,
                height: frame.size.height / 2.0,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::BottomCenterSixth => {
            let origin = CGPoint {
                x: frame.origin.x + frame.size.width / 3.0,
                y: frame.origin.y + frame.size.height / 2.0,
            };
            let size = CGSize {
                width: frame.size.width / 3.0,
                height: frame.size.height / 2.0,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::BottomRightSixth => {
            let origin = CGPoint {
                x: frame.origin.x + frame.size.width * 2.0 / 3.0,
                y: frame.origin.y + frame.size.height / 2.0,
            };
            let size = CGSize {
                width: frame.size.width / 3.0,
                height: frame.size.height / 2.0,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::TopThird => {
            let origin = frame.origin;
            let size = CGSize {
                width: frame.size.width,
                height: frame.size.height / 3.0,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::MiddleThird => {
            let origin = CGPoint {
                x: frame.origin.x,
                y: frame.origin.y + frame.size.height / 3.0,
            };
            let size = CGSize {
                width: frame.size.width,
                height: frame.size.height / 3.0,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::BottomThird => {
            let origin = CGPoint {
                x: frame.origin.x,
                y: frame.origin.y + frame.size.height * 2.0 / 3.0,
            };
            let size = CGSize {
                width: frame.size.width,
                height: frame.size.height / 3.0,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::Center => {
            let window_size = get_frontmost_window_size().unwrap().unwrap();
            let origin = CGPoint {
                x: frame.origin.x + (frame.size.width - window_size.width) / 2.0,
                y: frame.origin.y + (frame.size.height - window_size.height) / 2.0,
            };
            move_window(origin.x, origin.y).unwrap();
        }
        Command::FirstFourth => {
            let origin = frame.origin;
            let size = CGSize {
                width: frame.size.width / 4.0,
                height: frame.size.height,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::SecondFourth => {
            let origin = CGPoint {
                x: frame.origin.x + frame.size.width / 4.0,
                y: frame.origin.y,
            };
            let size = CGSize {
                width: frame.size.width / 4.0,
                height: frame.size.height,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::ThirdFourth => {
            let origin = CGPoint {
                x: frame.origin.x + frame.size.width * 2.0 / 4.0,
                y: frame.origin.y,
            };
            let size = CGSize {
                width: frame.size.width / 4.0,
                height: frame.size.height,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::LastFourth => {
            let origin = CGPoint {
                x: frame.origin.x + frame.size.width * 3.0 / 4.0,
                y: frame.origin.y,
            };
            let size = CGSize {
                width: frame.size.width / 4.0,
                height: frame.size.height,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::FirstThird => {
            let origin = CGPoint {
                x: frame.origin.x,
                y: frame.origin.y,
            };
            let size = CGSize {
                width: frame.size.width / 3.0,
                height: frame.size.height,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::CenterThird => {
            let origin = CGPoint {
                x: frame.origin.x + frame.size.width / 3.0,
                y: frame.origin.y,
            };
            let size = CGSize {
                width: frame.size.width / 3.0,
                height: frame.size.height,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::LastThird => {
            let origin = CGPoint {
                x: frame.origin.x + frame.size.width * 2.0 / 3.0,
                y: frame.origin.y,
            };
            let size = CGSize {
                width: frame.size.width / 3.0,
                height: frame.size.height,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::FirstTwoThirds => {
            let origin = CGPoint {
                x: frame.origin.x,
                y: frame.origin.y,
            };
            let size = CGSize {
                width: frame.size.width * 2.0 / 3.0,
                height: frame.size.height,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::CenterTwoThirds => {
            let origin = CGPoint {
                x: frame.origin.x + frame.size.width / 6.0,
                y: frame.origin.y,
            };
            let size = CGSize {
                width: frame.size.width * 2.0 / 3.0,
                height: frame.size.height,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::LastTwoThirds => {
            let origin = CGPoint {
                x: frame.origin.x + frame.size.width / 3.0,
                y: frame.origin.y,
            };
            let size = CGSize {
                width: frame.size.width * 2.0 / 3.0,
                height: frame.size.height,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::FirstThreeFourths => {
            let origin = CGPoint {
                x: frame.origin.x,
                y: frame.origin.y,
            };
            let size = CGSize {
                width: frame.size.width * 3.0 / 4.0,
                height: frame.size.height,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::CenterThreeFourths => {
            let origin = CGPoint {
                x: frame.origin.x + frame.size.width / 8.0,
                y: frame.origin.y,
            };
            let size = CGSize {
                width: frame.size.width * 3.0 / 4.0,
                height: frame.size.height,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::LastThreeFourths => {
            let origin = CGPoint {
                x: frame.origin.x + frame.size.width / 4.0,
                y: frame.origin.y,
            };
            let size = CGSize {
                width: frame.size.width * 3.0 / 4.0,
                height: frame.size.height,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::TopThreeFourths => {
            let origin = CGPoint {
                x: frame.origin.x,
                y: frame.origin.y,
            };
            let size = CGSize {
                width: frame.size.width,
                height: frame.size.height * 3.0 / 4.0,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::BottomThreeFourths => {
            let origin = CGPoint {
                x: frame.origin.x,
                y: frame.origin.y + frame.size.height / 4.0,
            };
            let size = CGSize {
                width: frame.size.width,
                height: frame.size.height * 3.0 / 4.0,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::TopTwoThirds => {
            let origin = CGPoint {
                x: frame.origin.x,
                y: frame.origin.y,
            };
            let size = CGSize {
                width: frame.size.width,
                height: frame.size.height * 2.0 / 3.0,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::BottomTwoThirds => {
            let origin = CGPoint {
                x: frame.origin.x,
                y: frame.origin.y + frame.size.height / 3.0,
            };
            let size = CGSize {
                width: frame.size.width,
                height: frame.size.height * 2.0 / 3.0,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }

        Command::TopCenterTwoThirds => {
            let origin = CGPoint {
                x: frame.origin.x + frame.size.width / 6.0,
                y: frame.origin.y,
            };
            let size = CGSize {
                width: frame.size.width * 2.0 / 3.0,
                height: frame.size.height * 2.0 / 3.0,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::TopFirstFourth => {
            let origin = CGPoint {
                x: frame.origin.x,
                y: frame.origin.y,
            };
            let size = CGSize {
                width: frame.size.width,
                height: frame.size.height / 4.0,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::TopSecondFourth => {
            let origin = CGPoint {
                x: frame.origin.x,
                y: frame.origin.y + frame.size.height / 4.0,
            };
            let size = CGSize {
                width: frame.size.width,
                height: frame.size.height / 4.0,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::TopThirdFourth => {
            let origin = CGPoint {
                x: frame.origin.x,
                y: frame.origin.y + frame.size.height * 2.0 / 4.0,
            };
            let size = CGSize {
                width: frame.size.width,
                height: frame.size.height / 4.0,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::TopLastFourth => {
            let origin = CGPoint {
                x: frame.origin.x,
                y: frame.origin.y + frame.size.height * 3.0 / 4.0,
            };
            let size = CGSize {
                width: frame.size.width,
                height: frame.size.height / 4.0,
            };
            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::MakeLarger => {
            let Some(window_origin) = get_frontmost_window_origin().unwrap() else {
                return;
            };
            let Some(window_size) = get_frontmost_window_size().unwrap() else {
                return;
            };
            let delta_width = 20_f64;
            let delta_height = window_size.height / window_size.width * delta_width;
            let delta_origin_x = delta_width / 2.0;
            let delta_origin_y = delta_height / 2.0;

            let new_width = {
                let possible_value = window_size.width + delta_width;
                if possible_value > frame.size.width {
                    frame.size.width
                } else {
                    possible_value
                }
            };
            let new_height = {
                let possible_value = window_size.height + delta_height;
                if possible_value > frame.size.height {
                    frame.size.height
                } else {
                    possible_value
                }
            };

            let new_origin_x = {
                let possible_value = window_origin.x - delta_origin_x;
                if possible_value < frame.origin.x {
                    frame.origin.x
                } else {
                    possible_value
                }
            };
            let new_origin_y = {
                let possible_value = window_origin.y - delta_origin_y;
                if possible_value < frame.origin.y {
                    frame.origin.y
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
            set_window_frame(new_frame).unwrap();
        }
        Command::MakeSmaller => {
            let Some(window_origin) = get_frontmost_window_origin().unwrap() else {
                return;
            };
            let Some(window_size) = get_frontmost_window_size().unwrap() else {
                return;
            };
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
            set_window_frame(new_frame).unwrap();
        }
        Command::AlmostMaximize => {
            let new_size = CGSize {
                width: frame.size.width * 0.8,
                height: frame.size.height * 0.8,
            };
            let new_origin = CGPoint {
                x: frame.origin.x + (frame.size.width * 0.1),
                y: frame.origin.y + (frame.size.height * 0.1),
            };
            let new_frame = CGRect {
                origin: new_origin,
                size: new_size,
            };
            set_window_frame(new_frame).unwrap();
        }
        Command::Maximize => {
            let new_frame = CGRect {
                origin: frame.origin,
                size: frame.size,
            };
            set_window_frame(new_frame).unwrap();
        }
        Command::MaximizeWidth => {
            let Some(window_size) = get_frontmost_window_size().unwrap() else {
                return;
            };
            let Some(window_origin) = get_frontmost_window_origin().unwrap() else {
                return;
            };
            let origin = CGPoint {
                x: frame.origin.x,
                y: window_origin.y,
            };
            let size = CGSize {
                width: frame.size.width,
                height: window_size.height,
            };

            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::MaximizeHeight => {
            let Some(window_size) = get_frontmost_window_size().unwrap() else {
                return;
            };
            let Some(window_origin) = get_frontmost_window_origin().unwrap() else {
                return;
            };
            let origin = CGPoint {
                x: window_origin.x,
                y: frame.origin.y,
            };
            let size = CGSize {
                width: window_size.width,
                height: frame.size.height,
            };

            let new_frame = CGRect { origin, size };
            set_window_frame(new_frame).unwrap();
        }
        Command::MoveUp => {
            let Some(window_origin) = get_frontmost_window_origin().unwrap() else {
                return;
            };
            let new_y = (window_origin.y - 10.0).max(frame.origin.y);
            move_window(window_origin.x, new_y).unwrap();
        }
        Command::MoveDown => {
            let Some(window_size) = get_frontmost_window_size().unwrap() else {
                return;
            };
            let Some(window_origin) = get_frontmost_window_origin().unwrap() else {
                return;
            };
            let new_y = (window_origin.y + 10.0)
                .min(frame.origin.y + frame.size.height - window_size.height);
            move_window(window_origin.x, new_y).unwrap();
        }
        Command::MoveLeft => {
            let Some(window_origin) = get_frontmost_window_origin().unwrap() else {
                return;
            };
            let new_x = (window_origin.x - 10.0).max(frame.origin.x);
            move_window(new_x, window_origin.y).unwrap();
        }
        Command::MoveRight => {
            let Some(window_size) = get_frontmost_window_size().unwrap() else {
                return;
            };
            let Some(window_origin) = get_frontmost_window_origin().unwrap() else {
                return;
            };
            let new_x =
                (window_origin.x + 10.0).min(frame.origin.x + frame.size.width - window_size.width);
            move_window(new_x, window_origin.y).unwrap();
        }
        Command::NextDesktop => {
            todo!()
        }
        Command::PreviousDesktop => {
            todo!()
        }
        Command::NextDisplay => {
            const TOO_MANY_MONITORS: &str = "I don't think you can have so many monitors";

            let frames = list_visible_frame_of_all_screens();
            let n_frames = frames.len();
            if n_frames <= 1 {
                return;
            }
            let index = frames
                .iter()
                .position(|fr| fr == &frame)
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

            set_window_frame(new_frame).unwrap();
        }
        Command::PreviousDisplay => {
            const TOO_MANY_MONITORS: &str = "I don't think you can have so many monitors";

            let frames = list_visible_frame_of_all_screens();
            let n_frames = frames.len();
            if n_frames <= 1 {
                return;
            }
            let index = frames
                .iter()
                .position(|fr| fr == &frame)
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

            set_window_frame(new_frame).unwrap();
        }
        Command::Restore => {
            todo!()
        }
        Command::ToggleFullscreen => {
            toggle_fullscreen().unwrap();
        }
    }
}
