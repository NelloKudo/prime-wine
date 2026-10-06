use crate::paths;
use crate::setup;
use std::process::Command;

const BRAVE_ARGS: [&str; 2] = [
    // opening brave as app standalone prevents it from breaking randomly on boot
    "--app=https://www.primevideo.com",
    // the windows storage apis needed for this are currently stubbed in wine and brave crashes
    "--disable-features=HardwareMediaKeyHandling",
];

pub fn launch_prime() -> Result<(), String> {
    let log_file = std::fs::File::create(paths::brave_log_file())
        .map_err(|e| format!("could not create log file: {}", e))?;
    let log_file_err = log_file
        .try_clone()
        .map_err(|e| format!("could not clone log file: {}", e))?;

    let mut cmd = Command::new(paths::wine_bin());
    cmd.arg(paths::brave_exe());
    cmd.args(BRAVE_ARGS);
    setup::add_wine_env(&mut cmd);
    cmd.env("WINEDEBUG", "-all");
    cmd.stdout(log_file);
    cmd.stderr(log_file_err);

    let started = std::time::Instant::now();
    let status = cmd
        .status()
        .map_err(|e| format!("could not start wine: {}", e))?;

    // dying this fast means something is broken
    if started.elapsed().as_secs() < 15 && !status.success() {
        return Err(format!(
            "brave closed right away, check the log at {}",
            paths::brave_log_file().display()
        ));
    }
    Ok(())
}

pub fn kill_wine() -> Result<(), String> {
    let mut cmd = Command::new(paths::wineserver_bin());
    cmd.arg("-k");
    setup::add_wine_env(&mut cmd);
    let status = cmd
        .status()
        .map_err(|e| format!("could not run wineserver: {}", e))?;
    if !status.success() {
        return Err("wineserver -k failed, maybe nothing was running".to_string());
    }
    Ok(())
}
