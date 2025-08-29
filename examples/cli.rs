use libwmgr::actions::Action;
use libwmgr::apply_to_focused_window;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cli <option>");
        std::process::exit(1);
    }

    let action_str = &args[1];
    let action = serde_plain::from_str::<Action>(action_str).unwrap_or_else(|error| {
        println!("Invalid command [{}], error [{}]", action_str, error);
        std::process::exit(1);
    });

    apply_to_focused_window(action).unwrap();
}
