// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use tauri::Manager;
use std::path::PathBuf;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn saudacao(nome: String) -> String {
    format!("Olá, {}, isso veio do Rust!", nome)
}

#[tauri::command]
fn download_video(url: String) -> Result<String, String> {
    use std::path::PathBuf;
    use std::process::Command;

    if url.is_empty() {
        return Err("Empty URL".into());
    }

    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let bin = base.join("bin");
    let ytdlp = bin.join("yt-dlp.exe");

    if !ytdlp.exists() {
        return Err(format!("yt-dlp not found in {}", ytdlp.display()));
    }

    // saves in downloads folder, which is created if it doesn't exist
    let out = Command::new(&ytdlp) //args for download
        .args([
            "--no-playlist",
            "--ffmpeg-location", &bin.to_string_lossy(),
            "-f", "bv*[ext=mp4][vcodec^=avc1]+ba[ext=m4a][acodec^=mp4a]/b[ext=mp4]",
            "--merge-output-format", "mp4",
            "--postprocessor-args", "ffmpeg:-c:a aac",
            "-o", "../downloads/%(title)s.%(ext)s",
            &url,
        ])
        .output()
        .map_err(|e| format!("Failed to run yt-dlp: {e}"))?;

    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    if !out.status.success() {
        return Err(format!("yt-dlp failed.\n\nstderr:\n{stderr}\n\nstdout:\n{stdout}"));
    }

    Ok(format!("OK\n\nstderr:\n{stderr}\n\nstdout:\n{stdout}"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, saudacao, download_video])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
