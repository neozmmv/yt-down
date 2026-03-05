# setup-bins.ps1
# Downloads yt-dlp.exe, ffmpeg.exe, and ffprobe.exe into src-tauri/bin/

$binDir = Join-Path $PSScriptRoot "src-tauri\bin"
New-Item -ItemType Directory -Force -Path $binDir | Out-Null

# --- yt-dlp ---
Write-Host "Downloading yt-dlp..." -ForegroundColor Cyan
$ytdlpDest = Join-Path $binDir "yt-dlp.exe"
Invoke-WebRequest `
    -Uri "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe" `
    -OutFile $ytdlpDest
Write-Host "  OK: yt-dlp.exe" -ForegroundColor Green

# --- ffmpeg + ffprobe (BtbN GPL build) ---
Write-Host "Downloading ffmpeg (this may take a minute)..." -ForegroundColor Cyan
$zipPath = Join-Path $env:TEMP "ffmpeg-win64.zip"
Invoke-WebRequest `
    -Uri "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-win64-gpl.zip" `
    -OutFile $zipPath

Write-Host "  Extracting ffmpeg.exe and ffprobe.exe..." -ForegroundColor Cyan
Add-Type -AssemblyName System.IO.Compression.FileSystem
$zip = [System.IO.Compression.ZipFile]::OpenRead($zipPath)

foreach ($entry in $zip.Entries) {
    if ($entry.Name -eq "ffmpeg.exe" -or $entry.Name -eq "ffprobe.exe") {
        $dest = Join-Path $binDir $entry.Name
        [System.IO.Compression.ZipFileExtensions]::ExtractToFile($entry, $dest, $true)
        Write-Host "  OK: $($entry.Name)" -ForegroundColor Green
    }
}
$zip.Dispose()
Remove-Item $zipPath

Write-Host "`nAll binaries ready in: $binDir" -ForegroundColor Green
