use serde::Serialize;
use std::fmt;
use std::fs::{self};

// ファイルシステムのアイテム
// 列挙型 FileSystemItem
// ファイル 又は ディレクトリ
#[derive(Serialize)]
pub enum FileSystemItem {
    FileItem(FileItem),           // File であれば、それは File 構造体のデータを持つ
    DirectoryItem(DirectoryItem), // Directory であれば、Directory 構造体のデータを持つ
}

#[derive(Serialize)]
pub struct FileItem {
    pub name: String,
    pub fullpath: String,
    pub size: u64,
}

#[derive(Serialize)]
pub struct DirectoryItem {
    pub name: String,
    pub fullpath: String,
    pub children: Vec<FileSystemItem>, //列挙型を持たせる
    pub size: u64,
}

impl FileSystemItem {

    #[allow(dead_code)]
    fn name(&self) -> &str {
        match self {
            //selfを評価
            FileSystemItem::FileItem(file) => &file.name, //Fileの場合は,fileでデータを束縛して、file.nameを返す
            FileSystemItem::DirectoryItem(dir) => &dir.name,
        }
    }

    fn size(&self) -> u64 {
        match self {
            FileSystemItem::FileItem(file) => file.size,
            FileSystemItem::DirectoryItem(dir) => {
                dir.children.iter().map(|child| child.size()).sum()
            }
        }
    }

    #[allow(dead_code)]
    fn is_directory(&self) -> bool {
        matches!(self, FileSystemItem::DirectoryItem(_))
    }
}

// Display トレイトの実装
// println! や format! マクロで値を文字列として表示する際に使用される
impl fmt::Display for FileSystemItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileSystemItem::FileItem(file) => {
                write!(f, "FileItem: {} , {} ", file.name, self.size())
            }
            FileSystemItem::DirectoryItem(dir) => {
                write!(
                    f,
                    "DirectoryItem: {} , {}, {}",
                    dir.name,
                    self.size(),
                    dir.children.len()
                )
            }
        }
    }
}

// [DEBUG用] ファイルシステムのアイテムを表示する関数
#[allow(dead_code)]
pub fn print_filesystem(item: &FileSystemItem, indent: usize) {
    println!("{}{}", "  ".repeat(indent), item);
    if let FileSystemItem::DirectoryItem(dir) = item {
        for child in &dir.children {
            print_filesystem(child, indent + 1);
        }
    }
}

#[allow(dead_code)]
pub fn print_dir_only(item: &DirectoryItem, indent: usize) {
    println!("{}{}", "  ".repeat(indent), &item.name);
    for child in item.children.iter() {
        if child.is_directory() {
            if let FileSystemItem::DirectoryItem(dir) = child {
                // `dir`を参照として扱う
                print_dir_only(&dir, indent + 1); // `&dir`を渡す
            }
        }
    }
}

/*
Rustの上記コードでは、find_directory関数がファイルシステムのツリー構造を再帰的に検索

match item  のパターンマッチングの説明
１，
FileSystemItem::Directory(dir): これは、itemがDirectory（ディレクトリ）型である場合にパターンマッチし、その時にdirという変数にバインドされる。
if dir.name == name: これはガード条件。ディレクトリの名前（dir.name）が検索している名前（name）と一致するかどうかをチェックします。
Some(item): ガード条件が真である場合、このディレクトリが探しているものなので、item（このディレクトリ自体）をSomeに包んで返します。

ガード条件とは、パターンマッチングの追加条件式のこと。
パターンマッチングが成功した後に、さらに条件を満たすかどうかをチェックするために使用される。

２，
FileSystemItem::Directory(dir): これは、itemがDirectory（ディレクトリ）型である場合にパターンマッチ。dirにバインド。
dir.children.iter().find_map(|child| find_directory(child, name)):

dir.childrenは、子アイテム。.iter()でイテレータを生成。
.find_map(|child| find_directory(child, name))
イテレータ上で各子アイテムに対して再帰的にfind_directory関数を呼び出す。

３，
_ => None: それ以外の場合は、Noneを返す。


ライフタイム
find_directory関数の再帰呼び出しは、ライフタイム指定が必要。
関数が参照を返すときに、返す参照が十分に長生きすることを保証する必要がある。
この場合、find_directory関数が再帰的に自分自身を呼び出すときに、返される参照が十分に長生きであると指示する。

find_directory<'a> というライフタイム指定は、ライフタイム'aを導入する。aは任意の名前で、ライフタイムパラメータと呼ばれる。
ライフタイムparameter 'a は、引数と返り値のライフタイムを関連付けるために使用される。
この 'a が付与されたものは、同じライフタイムを持つ参照であることを示す。
従って、find_directory関数は、itemとnameの参照を受け取り、itemのライフタイムがnameのライフタイムよりも長いことを示す。


*/
#[allow(dead_code)]
fn find_directory<'a>(item: &'a FileSystemItem, name: &str) -> Option<&'a FileSystemItem> {
    match item {
        FileSystemItem::DirectoryItem(dir) if dir.name == name => Some(item),
        FileSystemItem::DirectoryItem(dir) => dir
            .children
            .iter()
            .find_map(|child| find_directory(child, name)),
        _ => None,
    }
}

pub fn make_tree(file_info: &mut DirectoryItem) {
    match fs::read_dir(&file_info.fullpath) {
        Ok(entries) => {
            for entry in entries {
                let entry = entry.unwrap(); // ここで一度 unwrap して所有権の問題を解決
                let file_type = entry.file_type().unwrap();

                let name = entry.file_name().into_string().unwrap();
                let only_name = name.split('\\').last().unwrap_or("").to_string();

                if file_type.is_file() {
                    let file_size = match fs::metadata(entry.path()) {
                        // entryからPathBufを取得して渡す
                        Ok(metadata) => metadata.len(),
                        Err(e) => panic!("Failed to get file metadata: {}", e),
                    };

                    file_info.children.push(FileSystemItem::FileItem(FileItem {
                        name: only_name.to_string(),
                        fullpath: name.to_string(),
                        size: file_size,
                    }));
                }

                if file_type.is_dir() {
                    let dir_name = format!("{}\\{}", file_info.fullpath, &name);
                    let mut dir = DirectoryItem {
                        name: only_name.to_string(),
                        fullpath: dir_name,
                        children: Vec::new(),
                        size: 0, // 後でセット
                    };

                    make_tree(&mut dir);

                    let child = FileSystemItem::DirectoryItem(dir);
                    file_info.children.push(child);
                }
            }
        }
        Err(error_message) => {
            println!("エラー: {}, {}", error_message, &file_info.name);
        }
    }
}

pub fn get_json(data: &DirectoryItem) -> String {
    let json: String = serde_json::to_string_pretty(data).unwrap();

    return json;
}


pub fn calculate_directory_size(item: &mut DirectoryItem) -> u64 {
    let mut total_size = 0;
    for child in &mut item.children {
        match child {
            FileSystemItem::FileItem(file) => {
                total_size += file.size;
            },
            FileSystemItem::DirectoryItem(dir) => {
                let child_size = calculate_directory_size(dir);
                total_size += child_size;
            }
        }
    }
    item.size = total_size;
    total_size
}
