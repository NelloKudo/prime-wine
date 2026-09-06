use std::path::PathBuf;

// at some point i'll automate getting new versions, wanna stay safe from regressions for now
pub const WINE_VERSION: &str = "11.13";

fn home() -> PathBuf {
    let home = std::env::var("HOME").expect("HOME is not set");
    PathBuf::from(home)
}

pub fn data_dir() -> PathBuf {
    home().join(".local/share/prime-wine")
}

pub fn wine_dir() -> PathBuf {
    data_dir().join(format!("wine-{}-staging-amd64-wow64", WINE_VERSION))
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

pub fn winetricks_bin() -> PathBuf {
    data_dir().join("winetricks")
}

pub fn brave_log_file() -> PathBuf {
    data_dir().join("brave.log")
}

// wine runs the installer elevated so brave always lands in program files
pub fn brave_exe() -> PathBuf {
    prefix_dir().join("drive_c/Program Files/BraveSoftware/Brave-Browser/Application/brave.exe")
}

pub fn desktop_file() -> PathBuf {
    home().join(".local/share/applications/prime-wine.desktop")
}

pub fn manage_desktop_file() -> PathBuf {
    home().join(".local/share/applications/prime-wine-manage.desktop")
}

pub fn icon_file() -> PathBuf {
    home().join(".local/share/icons/hicolor/256x256/apps/prime-wine.png")
}

pub fn manage_icon_file() -> PathBuf {
    home().join(".local/share/icons/hicolor/256x256/apps/prime-wine-manage.png")
}

pub fn is_installed() -> bool {
    wine_bin().exists() && brave_exe().exists()
}
