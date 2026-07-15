use crate::download;
use crate::messages::WorkerMsg;
use crate::paths;
use crate::setup;
use std::process::Command;
use std::sync::mpsc::Sender;

// known good version, the update button grabs the latest instead
const BRAVE_INSTALLER_URL: &str = "https://github.com/brave/brave-browser/releases/download/v1.92.139/BraveBrowserStandaloneSetup.exe";
const BRAVE_INSTALLER_NAME: &str = "BraveBrowserStandaloneSetup.exe";
const BRAVE_RELEASES_API: &str = "https://api.github.com/repos/brave/brave-browser/releases/latest";

fn log(tx: &Sender<WorkerMsg>, text: &str) {
    let _ = tx.send(WorkerMsg::Log(text.to_string()));
}

fn installer_path() -> std::path::PathBuf {
    paths::data_dir().join(BRAVE_INSTALLER_NAME)
}

// runs the installer and waits until brave.exe shows up
fn run_installer(tx: &Sender<WorkerMsg>) -> Result<(), String> {
    log(tx, "running the brave installer...");
    let mut cmd = Command::new(paths::wine_bin());
    cmd.arg(installer_path());
    setup::add_wine_env(&mut cmd);
    // no pipes here, the installer hands them to helpers that never exit
    cmd.stdout(std::process::Stdio::null());
    cmd.stderr(std::process::Stdio::null());
    let mut child = cmd
        .spawn()
        .map_err(|e| format!("could not start the installer: {}", e))?;

    // the setup exe only exits once the install is really over
    log(tx, "waiting for the installer to finish...");
    let _ = child.wait();

    for _ in 0..60 {
        if paths::brave_exe().exists() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    if !paths::brave_exe().exists() {
        return Err("brave.exe never showed up, install failed".to_string());
    }

    // stop the leftover updater and the auto launched brave
    log(tx, "stopping leftover wine processes...");
    std::thread::sleep(std::time::Duration::from_secs(10));
    crate::launcher::kill_wine()?;
    Ok(())
}

pub fn install_brave(tx: &Sender<WorkerMsg>) -> Result<(), String> {
    if paths::brave_exe().exists() {
        log(tx, "brave is already installed, skipping");
        return Ok(());
    }
    log(tx, "downloading brave...");
    download::download_file(BRAVE_INSTALLER_URL, &installer_path(), tx)?;
    run_installer(tx)
}

// get latest stable release
fn latest_installer_url() -> Result<String, String> {
    let response = ureq::get(BRAVE_RELEASES_API)
        .set("User-Agent", "prime-wine")
        .call()
        .map_err(|e| format!("could not reach github: {}", e))?;

    let json: serde_json::Value = response
        .into_json()
        .map_err(|e| format!("bad answer from github: {}", e))?;

    let empty = Vec::new();
    let assets = json["assets"].as_array().unwrap_or(&empty);
    for asset in assets {
        let name = asset["name"].as_str().unwrap_or("");
        if name == BRAVE_INSTALLER_NAME {
            let url = asset["browser_download_url"].as_str().unwrap_or("");
            return Ok(url.to_string());
        }
    }
    Err("could not find the brave installer in the latest release".to_string())
}

// brave cannot update itself under wine so we redo the install with the newest one
pub fn update_brave(tx: &Sender<WorkerMsg>) -> Result<(), String> {
    log(tx, "checking the latest brave release...");
    let url = latest_installer_url()?;
    log(tx, &format!("downloading {}", url));
    download::download_file(&url, &installer_path(), tx)?;
    run_installer(tx)?;
    log(tx, "brave updated!");
    Ok(())
}
