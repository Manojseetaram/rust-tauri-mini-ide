"use client";

import { useState } from "react";
import Editor from "@monaco-editor/react";
import { invoke } from "@tauri-apps/api/core";

export default function CodeEditor() {
  const [code, setCode] = useState("// Start coding in Rust...");
  const [output, setOutput] = useState("");

  async function runCode() {
    try {
      const result = await invoke<string>("run_rust_code", { code });
      setOutput(result);
    } catch (err) {
      setOutput(`Error: ${err}`);
    }
  }

  return (
    <div style={{ height: "100vh", display: "flex", flexDirection: "column" }}>
      <Editor
        height="70%"
        defaultLanguage="rust"
        theme="vs-dark"
        value={code}
        onChange={(value) => setCode(value || "")}
      />
      <button onClick={runCode} style={{ padding: "10px", background: "#333", color: "white" }}>
        Run
      </button>
      <pre style={{ background: "#111", color: "white", padding: "10px", height: "30%" }}>
        {output}
      </pre>
    </div>
  );
}
