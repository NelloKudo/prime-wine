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

    let mut show_gui = is_manage || !paths::is_installed();
    let mut error = None;
    loop {
        if show_gui && !gui::run_gui(error.take()) {
            return;
        }
        match launcher::launch_prime() {
            Ok(()) => return,
            Err(e) => {
                error = Some(e);
                show_gui = true;
            }
        }
    }
}
