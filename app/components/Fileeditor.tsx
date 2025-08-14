"use client"; // also fix: should be `"use client"`, not `"use-client"`

import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export default function FileEditor() {
  const [path, setPath] = useState("");
  const [content, setContent] = useState("");
  const [status, setStatus] = useState("");

  async function handleRead() {
    try {
      const data = await invoke<string>("read_file", { path });
      setContent(data ?? "");
      setStatus("✅ File loaded");
    } catch (err) {
      setStatus(`❌ Error: ${err instanceof Error ? err.message : String(err)}`);
      console.error("Read file error:", err);
    }
  }

  async function handleWrite() {
    try {
      await invoke("write_file", { path, content });
      setStatus("💾 File saved");
    } catch (err) {
      setStatus(`❌ Error: ${err instanceof Error ? err.message : String(err)}`);
      console.error("Write file error:", err);
    }
  }

  return (
    <div style={{ padding: "1rem", fontFamily: "sans-serif" }}>
      <h1>🖥️ Manoj IDE</h1>

      <input
        type="text"
        placeholder="Enter file path..."
        value={path}
        onChange={(e) => setPath(e.target.value)}
        style={{ width: "100%", marginBottom: "0.5rem", padding: "0.5rem" }}
      />

      <div style={{ marginBottom: "0.5rem" }}>
        <button onClick={handleRead} style={{ marginRight: "0.5rem" }}>
          Read File
        </button>
        <button onClick={handleWrite}>Save File</button>
      </div>

      <textarea
        value={content}
        onChange={(e) => setContent(e.target.value)}
        style={{
          width: "100%",
          height: "300px",
          padding: "0.5rem",
          fontFamily: "monospace",
        }}
      />

      <p style={{ marginTop: "0.5rem" }}>{status}</p>
    </div>
  );
}
