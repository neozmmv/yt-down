// Prevents additional console window on Windows in release, DO NOT REMOVE!!

use std::path::PathBuf;
use tauri::Manager;

#[derive(serde::Serialize)]
struct VideoInfo {
    title: String,
    thumbnail: String,
}

fn ytdlp_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let resource_dir = app
        .path()
        .resource_dir()
        .map_err(|e| format!("Cannot find resource directory: {e}"))?;
    let path = resource_dir.join("bin").join("yt-dlp.exe");
    if !path.exists() {
        return Err(format!(
            "yt-dlp not found at {}. Please reinstall the application.",
            path.display()
        ));
    }
    Ok(path)
}

fn bin_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let resource_dir = app
        .path()
        .resource_dir()
        .map_err(|e| format!("Cannot find resource directory: {e}"))?;
    Ok(resource_dir.join("bin"))
}

#[tauri::command]
fn get_video_info(app: tauri::AppHandle, url: String) -> Result<VideoInfo, String> {
    use std::process::Command;

    if url.is_empty() {
        return Err("Empty URL".into());
    }

    let ytdlp = ytdlp_path(&app)?;

    let out = Command::new(&ytdlp)
        .args([
            "--no-playlist",
            "--dump-json",
            "--no-warnings",
            "--socket-timeout", "15",
            &url,
        ])
        .output()
        .map_err(|e| format!("Failed to run yt-dlp: {e}"))?;

    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(format!("Could not fetch video info: {}", stderr.trim()));
    }

    let json: serde_json::Value = serde_json::from_slice(&out.stdout)
        .map_err(|e| format!("Failed to parse video info: {e}"))?;

    let title = json["title"]
        .as_str()
        .unwrap_or("Unknown title")
        .to_string();

    // prefer a mid-res thumbnail to avoid giant images
    let thumbnail = json["thumbnails"]
        .as_array()
        .and_then(|arr| {
            // pick the last thumbnail with a reasonable width, or just the last one
            arr.iter()
                .filter(|t| t["url"].is_string())
                .max_by_key(|t| t["width"].as_u64().unwrap_or(0))
                .and_then(|t| t["url"].as_str())
        })
        .or_else(|| json["thumbnail"].as_str())
        .unwrap_or("")
        .to_string();

    Ok(VideoInfo { title, thumbnail })
}

#[tauri::command]
fn download_video(
    app: tauri::AppHandle,
    url: String,
    quality: String,
) -> Result<String, String> {
    use std::process::Command;

    if url.is_empty() {
        return Err("URL cannot be empty.".into());
    }

    let ytdlp = ytdlp_path(&app)?;
    let bin_str = bin_dir(&app)?.to_string_lossy().into_owned();

    let downloads_dir = app
        .path()
        .download_dir()
        .map_err(|e| format!("Cannot find Downloads folder: {e}"))?;

    let output_template = downloads_dir
        .join("%(title)s.%(ext)s")
        .to_string_lossy()
        .into_owned();

    let mut args: Vec<String> = vec![
        "--no-playlist".into(),
        "--ffmpeg-location".into(),
        bin_str,
    ];

    match quality.as_str() {
        "audio" => {
            args.extend([
                "-x".into(),
                "--audio-format".into(),
                "mp3".into(),
                "-o".into(),
                output_template,
                url,
            ]);
        }
        _ => {
            let height = match quality.as_str() {
                "480p" => "480",
                "720p" => "720",
                "1080p" => "1080",
                "best" => "9999",
                _ => "720",
            };
            let format = if height == "9999" {
                "bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]".to_string()
            } else {
                format!(
                    "bestvideo[height<={}][ext=mp4]+bestaudio[ext=m4a]/best[height<={}][ext=mp4]/best[ext=mp4]",
                    height, height
                )
            };
            args.extend([
                "-f".into(),
                format,
                "--merge-output-format".into(),
                "mp4".into(),
                "--postprocessor-args".into(),
                "ffmpeg:-c:a aac".into(),
                "-o".into(),
                output_template,
                url,
            ]);
        }
    }

    let out = Command::new(&ytdlp)
        .args(&args)
        .output()
        .map_err(|e| format!("Failed to run yt-dlp: {e}"))?;

    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        let stdout = String::from_utf8_lossy(&out.stdout);
        return Err(format!("{}\n{}", stderr.trim(), stdout.trim()));
    }

    Ok("Download complete!".into())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_video_info, download_video])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
