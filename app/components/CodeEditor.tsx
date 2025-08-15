"use client";

import { useState, useEffect } from "react";
import Editor from "@monaco-editor/react";
import { invoke } from "@tauri-apps/api/core";

interface CodeEditorProps {
  filePath: string;
}

export default function CodeEditor({ filePath }: CodeEditorProps) {
  const [code, setCode] = useState("");
  const [output, setOutput] = useState("");

  useEffect(() => {
    if (filePath) {
      invoke<string>("read_file", { path: filePath }).then(setCode);
    }
  }, [filePath]);

  const saveFile = async () => {
    if (!filePath) return;
    await invoke("write_file", { path: filePath, content: code });
    alert("File saved!");
  };

  const runCode = async () => {
    try {
      const result = await invoke<string>("run_rust_code", { code });
      setOutput(result);
    } catch (err) {
      setOutput(`Error: ${err}`);
    }
  };

  return (
    <div style={{ flex: 1, display: "flex", flexDirection: "column" }}>
      <div style={{ marginBottom: "5px" }}>
        <button onClick={saveFile}>💾 Save</button>
        <button onClick={runCode}>▶️ Run</button>
      </div>
      <Editor
        height="70%"
        defaultLanguage="rust"
        theme="vs-dark"
        value={code}
        onChange={v => setCode(v || "")}
      />
      <pre style={{ flex: 1, background: "#111", color: "white", padding: "10px" }}>
        {output}
      </pre>
    </div>
  );
}
