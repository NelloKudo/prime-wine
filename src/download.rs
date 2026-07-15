use crate::messages::WorkerMsg;
use std::io::{Read, Write};
use std::sync::mpsc::Sender;

// downloads a url into a file and reports progress to the gui
pub fn download_file(
    url: &str,
    dest: &std::path::Path,
    tx: &Sender<WorkerMsg>,
) -> Result<(), String> {
    let response = ureq::get(url)
        .call()
        .map_err(|e| format!("download failed: {}", e))?;

    let total: u64 = response
        .header("Content-Length")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);

    let mut reader = response.into_reader();
    let mut file =
        std::fs::File::create(dest).map_err(|e| format!("could not create file: {}", e))?;

    let mut buffer = [0u8; 65536];
    let mut downloaded: u64 = 0;
    loop {
        let n = reader
            .read(&mut buffer)
            .map_err(|e| format!("download read error: {}", e))?;
        if n == 0 {
            break;
        }
        file.write_all(&buffer[..n])
            .map_err(|e| format!("could not write file: {}", e))?;
        downloaded += n as u64;
        if total > 0 {
            let _ = tx.send(WorkerMsg::Progress(downloaded as f32 / total as f32));
        }
    }
    Ok(())
}
