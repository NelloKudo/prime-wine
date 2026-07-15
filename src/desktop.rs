use crate::paths;

const ICON_BYTES: &[u8] = include_bytes!("../assets/icon.png");
const MANAGE_ICON_BYTES: &[u8] = include_bytes!("../assets/icon-manage.png");

// the appimage runtime tells us where the appimage file is
fn app_path() -> String {
    if let Ok(path) = std::env::var("APPIMAGE") {
        return path;
    }
    match std::env::current_exe() {
        Ok(path) => path.display().to_string(),
        Err(_) => "prime-wine".to_string(),
    }
}

fn desktop_entry_text() -> String {
    let path = app_path();
    format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Name=Prime Video\n\
         GenericName=Video Streaming\n\
         Comment=Prime Video client alternative for Linux!\n\
         Exec=\"{path}\"\n\
         Icon=prime-wine\n\
         Terminal=false\n\
         Categories=AudioVideo;Video;Player;Network;\n\
         Keywords=prime;video;amazon;streaming;brave;wine;\n\
         Actions=manage;\n\
         \n\
         [Desktop Action manage]\n\
         Name=Manage Prime Video settings\n\
         Icon=prime-wine-manage\n\
         Exec=\"{path}\" --manage\n"
    )
}

// its own real entry so it also shows up when searching the menu
fn manage_desktop_entry_text() -> String {
    let path = app_path();
    format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Name=Manage Prime Video settings\n\
         Comment=Update Brave, reinstall or uninstall prime-wine\n\
         Exec=\"{path}\" --manage\n\
         Icon=prime-wine-manage\n\
         Terminal=false\n\
         Categories=AudioVideo;Settings;\n\
         Keywords=prime;video;wine;brave;update;settings;manage;\n"
    )
}

fn write_with_parent(path: &std::path::Path, contents: &[u8]) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("could not create folder: {}", e))?;
    }
    std::fs::write(path, contents).map_err(|e| format!("could not save {:?}: {}", path, e))
}

pub fn install_desktop_entry() -> Result<(), String> {
    write_with_parent(&paths::icon_file(), ICON_BYTES)?;
    write_with_parent(&paths::manage_icon_file(), MANAGE_ICON_BYTES)?;
    write_with_parent(&paths::desktop_file(), desktop_entry_text().as_bytes())?;
    write_with_parent(
        &paths::manage_desktop_file(),
        manage_desktop_entry_text().as_bytes(),
    )?;
    Ok(())
}

// rewrites the entries in case the appimage got moved or renamed
pub fn fix_desktop_entry() {
    let main = std::fs::read_to_string(paths::desktop_file()).unwrap_or_default();
    let manage = std::fs::read_to_string(paths::manage_desktop_file()).unwrap_or_default();
    if main != desktop_entry_text() || manage != manage_desktop_entry_text() {
        let _ = install_desktop_entry();
    }
}

pub fn remove_desktop_entry() {
    let _ = std::fs::remove_file(paths::desktop_file());
    let _ = std::fs::remove_file(paths::manage_desktop_file());
    let _ = std::fs::remove_file(paths::icon_file());
    let _ = std::fs::remove_file(paths::manage_icon_file());
}
