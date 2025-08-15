"use client";

import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { exists } from "@tauri-apps/plugin-fs";
import { Folder, File, Trash2, FolderOpen, FolderPlus } from "lucide-react";

interface FileExplorerProps {
  onOpenFile: (path: string) => void;
}

export default function FileExplorer({ onOpenFile }: FileExplorerProps) {
  const [currentPath, setCurrentPath] = useState<string>("");
  const [entries, setEntries] = useState<string[]>([]);
  const [recentFiles, setRecentFiles] = useState<string[]>([]);
  const [recentFolders, setRecentFolders] = useState<string[]>([]);

  const loadFolder = async (path: string) => {
    try {
      if (!path || !(await exists(path))) {
        setRecentFolders(prev => prev.filter(f => f !== path));
        return;
      }
      const folderEntries: string[] = await invoke("list_folder", { path });
      setEntries(folderEntries);
      setCurrentPath(path);
    } catch (err) {
      console.error("Failed to load folder:", err);
    }
  };

  const openFile = (path: string) => {
    onOpenFile(path);
    setRecentFiles(prev => [path, ...prev.filter(f => f !== path)].slice(0, 10));
  };

  const selectFolder = async () => {
    try {
      const folder = await open({ directory: true, multiple: false });
      if (folder && typeof folder === "string") {
        await loadFolder(folder);
        setRecentFolders(prev => [folder, ...prev.filter(f => f !== folder)].slice(0, 5));
      }
    } catch (err) {
      console.error("Failed to open folder dialog:", err);
    }
  };

  const deleteEntry = async (path: string) => {
    if (!confirm(`Delete ${path}?`)) return;
    try {
      await invoke("delete_path", { path });
      await loadFolder(currentPath);
    } catch (err) {
      console.error(err);
    }
  };

  const visibleEntries = entries.filter(f => !f.startsWith("."));

  useEffect(() => {
    const savedFiles = localStorage.getItem("recentFiles");
    if (savedFiles) setRecentFiles(JSON.parse(savedFiles));

    const savedFolders = localStorage.getItem("recentFolders");
    if (savedFolders) setRecentFolders(JSON.parse(savedFolders));
  }, []);

  useEffect(() => {
    localStorage.setItem("recentFiles", JSON.stringify(recentFiles));
  }, [recentFiles]);

  useEffect(() => {
    localStorage.setItem("recentFolders", JSON.stringify(recentFolders));
  }, [recentFolders]);

  return (
    <div className="w-72 bg-gray-900 border-r border-gray-700 p-4 text-white flex flex-col h-full">
      {/* Header */}
      <div className="flex justify-between items-center mb-4">
        <h2 className="text-lg font-bold">File Explorer</h2>
        <button
          onClick={selectFolder}
          className="flex items-center gap-1 bg-blue-600 hover:bg-blue-500 px-3 py-1 rounded text-sm"
        >
          <FolderPlus size={16} /> Open Folder
        </button>
      </div>

      {/* Current Path */}
      {currentPath && (
        <div className="mb-3 text-sm text-gray-300 truncate">
          <span className="font-semibold">Path:</span> {currentPath}
        </div>
      )}

      {/* Folder Contents */}
      <ul className="flex-1 overflow-y-auto space-y-1">
        {visibleEntries.map(entry => {
          const fullPath = `${currentPath}/${entry}`;
          const isFolder = entry.endsWith("/");

          return (
            <li
              key={fullPath}
              className="flex items-center justify-between px-2 py-1 hover:bg-gray-800 rounded"
            >
              <span
                className="flex items-center gap-2 cursor-pointer"
                onClick={() => (isFolder ? loadFolder(fullPath) : openFile(fullPath))}
              >
                {isFolder ? <Folder size={16} className="text-yellow-400" /> : <File size={16} />}
                {entry.replace(/\/$/, "")}
              </span>
            
            </li>
          );
        })}
      </ul>

      {/* Recent Folders */}
      {recentFolders.length > 0 && (
        <div className="mt-4">
          <h3 className="text-sm font-semibold text-gray-400 mb-1">Recent Folders</h3>
          <ul className="space-y-1">
            {recentFolders.map(f => (
              <li
                key={f}
                className="flex items-center justify-between px-2 py-1 hover:bg-gray-800 rounded"
              >
                <span
                  className="flex items-center gap-2 cursor-pointer"
                  onClick={() => loadFolder(f)}
                >
                  <FolderOpen size={16} className="text-yellow-300" /> {f.split("/").pop()}
                </span>
                <button
                  onClick={() => setRecentFolders(prev => prev.filter(rf => rf !== f))}
                  className="text-red-500 hover:text-red-400"
                >
                  <Trash2 size={16} />
                </button>
              </li>
            ))}
          </ul>
        </div>
      )}

      {/* Recent Files */}
      {recentFiles.length > 0 && (
        <div className="mt-4">
          <h3 className="text-sm font-semibold text-gray-400 mb-1">Recent Files</h3>
          <ul className="space-y-1">
            {recentFiles.map(f => (
              <li
                key={f}
                className="flex items-center justify-between px-2 py-1 hover:bg-gray-800 rounded"
              >
                <span
                  className="flex items-center gap-2 cursor-pointer"
                  onClick={() => openFile(f)}
                >
                  <File size={16} className="text-green-400" /> {f.split("/").pop()}
                </span>
                <button
                  onClick={() => setRecentFiles(prev => prev.filter(rf => rf !== f))}
                  className="text-red-500 hover:text-red-400"
                >
                  <Trash2 size={16} />
                </button>
              </li>
            ))}
          </ul>
        </div>
      )}
    </div>
  );
}
