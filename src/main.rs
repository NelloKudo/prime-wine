mod brave;
mod desktop;
mod download;
mod gui;
mod launcher;
mod messages;
mod paths;
mod setup;
mod theme;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let is_manage = args.iter().any(|a| a == "--manage");

    // first run or explicit request opens the manager window
    if is_manage || !paths::is_installed() {
        gui::run_gui(None);
        return;
    }

    // keep the menu entry pointing at us in case the appimage moved
    desktop::fix_desktop_entry();

    // normal click goes straight to prime video
    if let Err(e) = launcher::launch_prime_and_wait() {
        gui::run_gui(Some(e));
    }
}
