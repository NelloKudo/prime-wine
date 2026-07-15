use std::path::PathBuf;

// at some point i'll automate getting new versions, wanna stay safe from regressions for now
pub const WINE_FOLDER_NAME: &str = "wine-11.13-staging-amd64-wow64";

pub fn data_dir() -> PathBuf {
    let home = std::env::var("HOME").expect("HOME is not set");
    PathBuf::from(home).join(".local/share/prime-wine")
}

pub fn wine_dir() -> PathBuf {
    data_dir().join(WINE_FOLDER_NAME)
}

pub fn wine_bin() -> PathBuf {
    wine_dir().join("bin/wine")
}

pub fn wineserver_bin() -> PathBuf {
    wine_dir().join("bin/wineserver")
}

pub fn prefix_dir() -> PathBuf {
    data_dir().join("prefix")
}

fn user_brave_exe() -> PathBuf {
    let user = std::env::var("USER").unwrap_or_else(|_| "user".to_string());
    prefix_dir().join(format!(
        "drive_c/users/{}/AppData/Local/BraveSoftware/Brave-Browser/Application/brave.exe",
        user
    ))
}

fn system_brave_exe() -> PathBuf {
    prefix_dir().join("drive_c/Program Files/BraveSoftware/Brave-Browser/Application/brave.exe")
}

// the silent installer installs per user, the normal one goes to program files
pub fn brave_exe() -> PathBuf {
    let user_path = user_brave_exe();
    if user_path.exists() {
        return user_path;
    }
    let system_path = system_brave_exe();
    if system_path.exists() {
        return system_path;
    }
    user_path
}

pub fn winetricks_bin() -> PathBuf {
    data_dir().join("winetricks")
}

pub fn brave_log_file() -> PathBuf {
    data_dir().join("brave.log")
}

pub fn desktop_file() -> PathBuf {
    let home = std::env::var("HOME").expect("HOME is not set");
    PathBuf::from(home).join(".local/share/applications/prime-wine.desktop")
}

pub fn icon_file() -> PathBuf {
    let home = std::env::var("HOME").expect("HOME is not set");
    PathBuf::from(home).join(".local/share/icons/hicolor/256x256/apps/prime-wine.png")
}

pub fn manage_desktop_file() -> PathBuf {
    let home = std::env::var("HOME").expect("HOME is not set");
    PathBuf::from(home).join(".local/share/applications/prime-wine-manage.desktop")
}

pub fn manage_icon_file() -> PathBuf {
    let home = std::env::var("HOME").expect("HOME is not set");
    PathBuf::from(home).join(".local/share/icons/hicolor/256x256/apps/prime-wine-manage.png")
}

pub fn is_installed() -> bool {
    wine_bin().exists() && brave_exe().exists()
}
