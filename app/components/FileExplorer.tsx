"use client";

import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open as openDialog } from "@tauri-apps/api/dialog";

interface FileExplorerProps {
  onOpenFile: (path: string) => void;
}

export default function FileExplorer({ onOpenFile }: FileExplorerProps) {
  const [currentPath, setCurrentPath] = useState<string>("");
  const [entries, setEntries] = useState<string[]>([]);
  const [recentFiles, setRecentFiles] = useState<string[]>([]);

  const loadFolder = async (path: string) => {
    try {
      const folderEntries: string[] = await invoke("list_folder", { path });
      setEntries(folderEntries);
      setCurrentPath(path);
    } catch (err) {
      console.error("Failed to load folder:", err);
    }
  };

  const openFile = (path: string) => {
    onOpenFile(path);
    setRecentFiles((prev) => [path, ...prev.filter((f) => f !== path)].slice(0, 10));
  };

  const selectFolder = async () => {
    try {
      const folder = await openDialog({ directory: true });
      if (folder) await loadFolder(folder as string);
    } catch (err) {
      console.error("Failed to select folder:", err);
    }
  };

  const createNewFile = async () => {
    if (!currentPath) return alert("Select a folder first");
    const name = prompt("Enter file name");
    if (!name) return;
    const newPath = `${currentPath}/${name}`;
    try {
      await invoke("create_file", { path: newPath });
      await loadFolder(currentPath);
    } catch (err) {
      console.error("Failed to create file:", err);
    }
  };

  const createNewFolder = async () => {
    if (!currentPath) return alert("Select a folder first");
    const name = prompt("Enter folder name");
    if (!name) return;
    const newPath = `${currentPath}/${name}`;
    try {
      await invoke("create_folder", { path: newPath });
      await loadFolder(currentPath);
    } catch (err) {
      console.error("Failed to create folder:", err);
    }
  };

  const deleteEntry = async (path: string) => {
    if (!confirm(`Delete ${path}?`)) return;
    try {
      await invoke("delete_path", { path });
      await loadFolder(currentPath);
    } catch (err) {
      console.error("Failed to delete:", err);
    }
  };

  const visibleEntries = entries.filter((f) => !f.startsWith("."));

  useEffect(() => {
    const saved = localStorage.getItem("recentFiles");
    if (saved) setRecentFiles(JSON.parse(saved));
  }, []);

  useEffect(() => {
    localStorage.setItem("recentFiles", JSON.stringify(recentFiles));
  }, [recentFiles]);

  return (
    <div style={{ width: "300px", borderRight: "1px solid #333", padding: "10px", color: "white" }}>
      <h3>Explorer</h3>
      <div style={{ marginBottom: "10px" }}>
        <button onClick={selectFolder} style={{ marginRight: "5px" }}>Select Folder</button>
        <button onClick={createNewFile} disabled={!currentPath} style={{ marginRight: "5px" }}>New File</button>
        <button onClick={createNewFolder} disabled={!currentPath}>New Folder</button>
      </div>

      {currentPath && <h4>Path: {currentPath}</h4>}

      <ul style={{ padding: 0, listStyle: "none" }}>
        {visibleEntries.map((entry) => {
          const fullPath = `${currentPath}/${entry}`;
          const isFolder = entry.endsWith("/");
          return (
            <li key={fullPath} style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: "4px" }}>
              <span
                style={{ cursor: "pointer", color: isFolder ? "lightblue" : "white" }}
                onClick={() => (isFolder ? loadFolder(fullPath) : openFile(fullPath))}
              >
                {entry.replace(/\/$/, "")}
              </span>
              <button onClick={() => deleteEntry(fullPath)}>🗑️</button>
            </li>
          );
        })}
      </ul>

      {recentFiles.length > 0 && (
        <>
          <h4>Recent Files</h4>
          <ul style={{ padding: 0, listStyle: "none" }}>
            {recentFiles.map((f) => (
              <li key={f}>
                <span style={{ cursor: "pointer", color: "lightgreen" }} onClick={() => openFile(f)}>
                  {f.split("/").pop()}
                </span>
              </li>
            ))}
          </ul>
        </>
      )}
    </div>
  );
}
