"use client";

import { useState } from "react";
import FileExplorer from "./components/FileExplorer";
import CodeEditor from "./components/CodeEditor";


export default function IDEPage() {
  const [currentFile, setCurrentFile] = useState("");

  return (
    <div style={{ display: "flex", height: "100vh" }}>
      <FileExplorer onOpenFile={setCurrentFile} />

      {currentFile ? (
        <CodeEditor filePath={currentFile} />
      ) : (
        <div style={{ flex: 1, display: "flex", alignItems: "center", justifyContent: "center", color: "#888" }}>
          Select a file to start editing
        </div>
      )}
    </div>
  );
}
