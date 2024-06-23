import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import FileSystemView, { DirectoryItem, FileItem, FileSystemItem } from "./DirectoryView";

function App() {
  const [targetDir, setTargetDir] = useState("");

  async function getdir() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

    const data: string = await invoke("getdir");
    if (data === "") {
      console.log("select canceled");
      return;
    }
    setTargetDir(data);
  }

  function parseFileSystemData(jsonString: string): FileSystemItem[] {
    const parseItem = (item: any): FileSystemItem => {
      if ('FileItem' in item) {
        return {
          type: 'FileItem',
          data: item.FileItem as FileItem
        };
      } else if ('DirectoryItem' in item) {
        return {
          type: 'DirectoryItem',
          data: {
            ...item.DirectoryItem,
            //children: item.DirectoryItem.children.map(parseItem)
            children: Array.isArray(item.DirectoryItem.children) ? item.DirectoryItem.children.map(parseItem) : []
          } as DirectoryItem
        };
      }
      console.error('Invalid item:', JSON.stringify(item, null, 2));
      throw new Error('Invalid item type: ' + JSON.stringify(item));
    };

    try {
      const parsedData = JSON.parse(jsonString);
      if ('DirectoryItem' in parsedData) {
        return [parseItem({ DirectoryItem: parsedData })];
      } else if (Array.isArray(parsedData)) {
        return parsedData.map(parseItem);
      } else {
        return [parseItem({ DirectoryItem: parsedData })];
        //throw new Error('Invalid JSON structure');
      }
    } catch (error) {
      console.error('Error parsing JSON:', error);
      return [];
    }
  }
  const fileSystemData: FileSystemItem[] = parseFileSystemData(targetDir);

  return (
    <div className="container">
      <h1>Folder Size Viewer</h1>
      <p>各フォルダのデータ量を表示</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          getdir();
        }}
      >
        <button type="submit">フォルダ選択</button>
      </form>

      <FileSystemView items={fileSystemData} />
    </div>
  );
}

export default App;


