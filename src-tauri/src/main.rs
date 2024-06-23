// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod item;
use item::{calculate_directory_size, get_json, make_tree, DirectoryItem};
use tauri::api::dialog::blocking::FileDialogBuilder;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![getdir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn getdir() -> String {
    let path_name = choose_dir();
    if path_name == "" {
        return "".to_string();
    }

    let mut root = DirectoryItem {
        name: path_name.to_string(),
        fullpath: path_name.to_string(),
        children: Vec::new(),
        size: 0, //後でセット
    };
    make_tree(&mut root);

    //Directoryのsizeを計算して設定
    calculate_directory_size(&mut root);

    //print_dir_only(&root, 0);
    let json: String = get_json(&root);

    json
}

fn choose_dir() -> String {
    let file_paths = FileDialogBuilder::new().pick_folders();
    match file_paths {
        Some(v) => {
            let mut pathstr = "".to_string();
            // 対象はディレクトリ一つ（最初のひとつ）とする
            match v.first() {
                Some(path) => match path.to_str() {
                    Some(s) => {
                        pathstr = s.to_string();
                    }
                    _ => {}
                },
                _ => {}
            }

            return pathstr;
        }
        _ => {
            return "".to_string();
        }
    }
}
