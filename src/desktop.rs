use crate::paths;
use std::path::Path;

const ICON_BYTES: &[u8] = include_bytes!("../assets/icon.png");
const MANAGE_ICON_BYTES: &[u8] = include_bytes!("../assets/icon-manage.png");

fn app_path() -> String {
    if let Ok(path) = std::env::var("APPIMAGE") {
        return path;
    }
    match std::env::current_exe() {
        Ok(path) => path.display().to_string(),
        Err(_) => "prime-wine".to_string(),
    }
}

fn save_file(path: &Path, contents: &[u8]) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("could not create folder: {}", e))?;
    }
    std::fs::write(path, contents).map_err(|e| format!("could not save {:?}: {}", path, e))
}

pub fn install_desktop_entry() -> Result<(), String> {
    let path = app_path();

    let main = format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Name=Prime Video\n\
         GenericName=Video Streaming\n\
         Comment=Prime Video client alternative for Linux!\n\
         Exec=\"{path}\"\n\
         Icon=prime-wine\n\
         Terminal=false\n\
         Categories=AudioVideo;Video;Player;Network;\n\
         Keywords=prime;video;amazon;streaming;brave;wine;\n"
    );

    let manage = format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Name=Manage Prime Video settings\n\
         Comment=Update Brave, reinstall or uninstall prime-wine\n\
         Exec=\"{path}\" --manage\n\
         Icon=prime-wine-manage\n\
         Terminal=false\n\
         Categories=AudioVideo;Settings;\n\
         Keywords=prime;video;wine;brave;update;settings;manage;\n"
    );

    save_file(&paths::icon_file(), ICON_BYTES)?;
    save_file(&paths::manage_icon_file(), MANAGE_ICON_BYTES)?;
    save_file(&paths::desktop_file(), main.as_bytes())?;
    save_file(&paths::manage_desktop_file(), manage.as_bytes())?;
    Ok(())
}

pub fn remove_desktop_entry() {
    for file in [
        paths::desktop_file(),
        paths::manage_desktop_file(),
        paths::icon_file(),
        paths::manage_icon_file(),
    ] {
        let _ = std::fs::remove_file(file);
    }
}
