import { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

type Quality = "480p" | "720p" | "1080p" | "best" | "audio";

const QUALITY_OPTIONS: { value: Quality; label: string }[] = [
  { value: "480p", label: "480p" },
  { value: "720p", label: "720p" },
  { value: "1080p", label: "1080p" },
  { value: "best", label: "Best" },
  { value: "audio", label: "MP3" },
];

interface VideoInfo {
  title: string;
  thumbnail: string;
}

export default function App() {
  const [url, setUrl] = useState("");
  const [quality, setQuality] = useState<Quality>("720p");
  const [status, setStatus] = useState<
    "idle" | "loading" | "success" | "error"
  >("idle");
  const [errorMessage, setErrorMessage] = useState("");
  const [videoInfo, setVideoInfo] = useState<VideoInfo | null>(null);
  const [infoLoading, setInfoLoading] = useState(false);

  // debounce ref to cancel stale fetches
  const debounceTimer = useRef<ReturnType<typeof setTimeout> | null>(null);
  const latestUrl = useRef("");

  useEffect(() => {
    const trimmed = url.trim();

    // clear info if url is cleared
    if (!trimmed) {
      setVideoInfo(null);
      setInfoLoading(false);
      return;
    }

    setInfoLoading(true);
    setVideoInfo(null);

    if (debounceTimer.current) clearTimeout(debounceTimer.current);

    debounceTimer.current = setTimeout(async () => {
      latestUrl.current = trimmed;
      try {
        const info = await invoke<VideoInfo>("get_video_info", {
          url: trimmed,
        });
        // only update if url hasn't changed while fetching
        if (latestUrl.current === trimmed) {
          setVideoInfo(info);
        }
      } catch {
        // silently ignore info errors — don't block the user from downloading
        if (latestUrl.current === trimmed) {
          setVideoInfo(null);
        }
      } finally {
        if (latestUrl.current === trimmed) {
          setInfoLoading(false);
        }
      }
    }, 800);

    return () => {
      if (debounceTimer.current) clearTimeout(debounceTimer.current);
    };
  }, [url]);

  const handleDownload = async () => {
    if (!url.trim()) return;
    setStatus("loading");
    setErrorMessage("");
    try {
      await invoke<string>("download_video", { url: url.trim(), quality });
      setStatus("success");
    } catch (err) {
      setStatus("error");
      setErrorMessage(String(err));
    }
  };

  return (
    <main className="container">
      <h1>YouTube Media Downloader</h1>

      <div className="input-row">
        <input
          type="text"
          placeholder="Paste YouTube URL..."
          value={url}
          onChange={(e) => {
            setUrl(e.target.value);
            setStatus("idle");
          }}
          onKeyDown={(e) => e.key === "Enter" && handleDownload()}
          disabled={status === "loading"}
        />
      </div>

      {(infoLoading || videoInfo) && (
        <div className="video-preview">
          {infoLoading && !videoInfo && (
            <div className="preview-skeleton">
              <div className="skeleton-thumb" />
              <div className="skeleton-title" />
            </div>
          )}
          {videoInfo && (
            <>
              {videoInfo.thumbnail && (
                <img
                  className="preview-thumb"
                  src={videoInfo.thumbnail}
                  alt={videoInfo.title}
                />
              )}
              <p className="preview-title">{videoInfo.title}</p>
            </>
          )}
        </div>
      )}

      <div className="quality-row">
        {QUALITY_OPTIONS.map((opt) => (
          <button
            key={opt.value}
            className={`quality-btn${quality === opt.value ? " active" : ""}`}
            onClick={() => setQuality(opt.value)}
            disabled={status === "loading"}
          >
            {opt.label}
          </button>
        ))}
      </div>

      <button
        className="download-btn"
        onClick={handleDownload}
        disabled={status === "loading" || !url.trim()}
      >
        {status === "loading" ? "Downloading..." : "Download"}
      </button>

      {status === "success" && (
        <p className="status success">Saved to your Downloads folder.</p>
      )}
      {status === "error" && (
        <details className="status error">
          <summary>Download failed</summary>
          <pre>{errorMessage}</pre>
        </details>
      )}
    </main>
  );
}
