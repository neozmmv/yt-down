# YouTube Media Downloader

A simple YouTube downloader built with Tauri + React. Supports video quality selection (480p, 720p, 1080p, Best) and audio-only (MP3) downloads.

Downloads are saved to your system **Downloads** folder (`C:\Users\<you>\Downloads`).

---

## Requirements

- [Node.js](https://nodejs.org/) (v18+) and [pnpm](https://pnpm.io/)
- [Rust](https://rustup.rs/) (stable toolchain)
- [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) (WebView2 on Windows)

---

## Setup

### 1. Install Node dependencies

```powershell
pnpm install
```

### 2. Download yt-dlp, ffmpeg, and ffprobe

Run the provided setup script from the project root. It downloads the binaries and places them in `src-tauri/bin/`:

```powershell
.\setup-bins.ps1
```

> If PowerShell blocks the script, run this first:
>
> ```powershell
> Set-ExecutionPolicy -Scope CurrentUser RemoteSigned
> ```

After the script finishes you should have:

```
src-tauri/bin/
  yt-dlp.exe
  ffmpeg.exe
  ffprobe.exe
```

### 3. Run in development

```powershell
pnpm tauri dev
```

---

## Build

```powershell
pnpm tauri build
```

---

## Usage

1. Paste a YouTube URL into the input field.
2. The video title and thumbnail will appear automatically.
3. Select a quality: **480p**, **720p**, **1080p**, **Best**, or **MP3**.
4. Click **Download**. The file is saved to your Downloads folder.

## Disclaimer

This application was made for Windows. Not tested or built on Linux or MacOS.
