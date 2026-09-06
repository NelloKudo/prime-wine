mod brave;
mod desktop;
mod download;
mod gui;
mod launcher;
mod paths;
mod setup;
mod theme;

fn main() {
    let is_manage = std::env::args().any(|a| a == "--manage");

    // first run or explicit request opens the manager window
    if is_manage || !paths::is_installed() {
        gui::run_gui(None);
        return;
    }

    // normal click goes straight to prime video
    if let Err(e) = launcher::launch_prime_and_wait() {
        gui::run_gui(Some(e));
    }
}
