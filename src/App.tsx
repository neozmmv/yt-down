import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import { useForm } from "react-hook-form";
import "./App.css";

export default function App() {
  const { register, handleSubmit } = useForm<{ url: string }>();
  const [name, setName] = useState<string>("");
  const chamarRust = async (data: { url: string }) => {
    const res = await invoke<string>("download_video", {
      url: data.url,
    });
    setName(res);
  };

  function downloadVideo(data: { url: string }) {
    chamarRust(data);
  }
  return (
    <main className="container">
      <h1>YouTube Downloader</h1>
      <form onSubmit={handleSubmit(downloadVideo)}>
        <input
          type="text"
          placeholder="Enter YouTube URL"
          {...register("url")}
        />
      </form>
    </main>
  );
}
