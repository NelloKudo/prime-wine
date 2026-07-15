use crate::download;
use crate::messages::WorkerMsg;
use crate::paths;
use std::io::BufRead;
use std::os::unix::fs::PermissionsExt;
use std::process::{Command, Stdio};
use std::sync::mpsc::Sender;

const WINE_URL: &str = "https://github.com/Kron4ek/Wine-Builds/releases/download/11.13/wine-11.13-staging-amd64-wow64.tar.xz";
const WINETRICKS_URL: &str =
    "https://raw.githubusercontent.com/Winetricks/winetricks/master/src/winetricks";
const WINETRICKS_PACKAGES: &str = "dxvk vkd3d corefonts vcrun2022 win10";

fn log(tx: &Sender<WorkerMsg>, text: &str) {
    let _ = tx.send(WorkerMsg::Log(text.to_string()));
}

// path with our wine and the tools bundled inside the appimage
fn full_path_var() -> String {
    let mut path = std::env::var("PATH").unwrap_or_default();
    path = format!("{}:{}", paths::wine_dir().join("bin").display(), path);
    if let Ok(appdir) = std::env::var("APPDIR") {
        path = format!("{}/usr/bin:{}", appdir, path);
    }
    path
}

// adds the wine environment to a command
pub fn add_wine_env(cmd: &mut Command) {
    cmd.env("WINEPREFIX", paths::prefix_dir());
    cmd.env("WINE", paths::wine_bin());
    cmd.env("PATH", full_path_var());
    // no start menu or desktop entries from wine
    cmd.env("WINEDLLOVERRIDES", "winemenubuilder.exe=d");
}

// runs a command and sends its output lines to the gui
pub fn run_logged_command(mut cmd: Command, tx: &Sender<WorkerMsg>) -> Result<(), String> {
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child = cmd
        .spawn()
        .map_err(|e| format!("could not start command: {}", e))?;

    // stderr goes through a second thread so nothing gets stuck
    let stderr = child.stderr.take().unwrap();
    let tx_err = tx.clone();
    let err_thread = std::thread::spawn(move || {
        let reader = std::io::BufReader::new(stderr);
        for line in reader.lines().map_while(Result::ok) {
            let _ = tx_err.send(WorkerMsg::Log(line));
        }
    });

    let stdout = child.stdout.take().unwrap();
    let reader = std::io::BufReader::new(stdout);
    for line in reader.lines().map_while(Result::ok) {
        let _ = tx.send(WorkerMsg::Log(line));
    }

    let _ = err_thread.join();
    let status = child
        .wait()
        .map_err(|e| format!("could not wait for command: {}", e))?;

    if !status.success() {
        return Err(format!("command exited with {}", status));
    }
    Ok(())
}

fn download_and_extract_wine(tx: &Sender<WorkerMsg>) -> Result<(), String> {
    if paths::wine_bin().exists() {
        log(tx, "wine is already there, skipping");
        return Ok(());
    }

    let archive = paths::data_dir().join("wine.tar.xz");
    log(tx, "downloading wine 11.13 staging...");
    download::download_file(WINE_URL, &archive, tx)?;

    log(tx, "extracting wine...");
    let mut cmd = Command::new("tar");
    cmd.arg("-xJf");
    cmd.arg(&archive);
    cmd.arg("-C");
    cmd.arg(paths::data_dir());
    run_logged_command(cmd, tx)?;

    let _ = std::fs::remove_file(&archive);

    if !paths::wine_bin().exists() {
        return Err("wine binary is missing after extraction".to_string());
    }
    Ok(())
}

fn download_winetricks(tx: &Sender<WorkerMsg>) -> Result<(), String> {
    log(tx, "downloading winetricks...");
    download::download_file(WINETRICKS_URL, &paths::winetricks_bin(), tx)?;

    // make it executable
    let perms = std::fs::Permissions::from_mode(0o755);
    std::fs::set_permissions(paths::winetricks_bin(), perms)
        .map_err(|e| format!("could not chmod winetricks: {}", e))?;
    Ok(())
}

fn setup_prefix(tx: &Sender<WorkerMsg>) -> Result<(), String> {
    log(tx, "setting up the wine prefix, this takes a while...");
    let mut cmd = Command::new("bash");
    cmd.arg(paths::winetricks_bin());
    cmd.arg("-q");
    cmd.arg("-f");
    for package in WINETRICKS_PACKAGES.split(' ') {
        cmd.arg(package);
    }
    add_wine_env(&mut cmd);
    run_logged_command(cmd, tx)?;
    Ok(())
}

fn check_host_tools(tx: &Sender<WorkerMsg>) -> Result<(), String> {
    // winetricks needs these, cabextract comes bundled in the appimage
    for tool in ["bash", "tar", "xz", "cabextract"] {
        let found = Command::new("which")
            .arg(tool)
            .env("PATH", full_path_var())
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
        if !found {
            return Err(format!("please install '{}' from your distro first", tool));
        }
        log(tx, &format!("found {}", tool));
    }
    Ok(())
}

// the whole install, runs in a worker thread
pub fn run_install(tx: &Sender<WorkerMsg>) -> Result<(), String> {
    std::fs::create_dir_all(paths::data_dir())
        .map_err(|e| format!("could not create data folder: {}", e))?;

    check_host_tools(tx)?;
    download_and_extract_wine(tx)?;
    download_winetricks(tx)?;
    setup_prefix(tx)?;
    crate::brave::install_brave(tx)?;
    crate::desktop::install_desktop_entry()?;

    log(tx, "all done!");
    Ok(())
}
