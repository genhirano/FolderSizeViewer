# Folder Size Viewer
**フォルダを指定して、そのフォルダと、配下フォルダのデータ量を階層表示するアプリ**  
for Windows (Windowsネイティブアプリ)
## このプロジェクトは・・
**Windowsデスクトップアプリをひとつ作ってみる。**

私は、[TurboPascal](https://ja.wikipedia.org/wiki/Turbo_Pascal),　[Delphi](https://ja.wikipedia.org/wiki/Delphi)でしかWindowsネイティブアプリを作ったことがない。
現在、Windowsネイティブ／クロスプラットフォームアプリ開発を勉強中。

このプロジェクトで、[Tauri](https://tauri.app/)を使ってWindowsネイティブアプリをつくってみて感覚を掴む。

![image](https://github.com/genhirano/FolderSizeViewer/assets/3538386/7db354b4-45ac-459c-a819-3df9c7df2fe2)

> [!NOTE]
> バイナリ配布しますが、利用は自己責任にてお願いします。

## ゴール
  * Windowsネイティブアプリ(.exe)を作る
  * Tauriの基本概念を大まかに把握する(OSネイティブとWebviewの通信など)
  * Rust,React,TypeScriptに触れ、ちょっと上達する
## 制作物のテーマ
* **「Windowsのフォルダを指定して、そのフォルダと、配下のフォルダのデータ量を階層で表示するアプリ」**
  * Hello World!よりちょっと難しいくらいのものを作る
  * 基本的な再帰処理を体験するため、シンプルなディレクトリ構造をデータに持つ
  * Rustでは、構造体(struct)に参照をもたせると所有権地獄なので、（双方向リスト構造は避けて）ディレクトリ構造をテーマにして実装してみる。
    * もし複雑になりすぎる場合は、子オブジェクトに親オブジェクトへの参照は持たせない。
    * 後日談：下階層から親を指す「parent」は地獄。Rust初心者にはまだ早い。
## 使用技術
### Tauri : Rust + (React + Typescript)
* Tauri(フレームワーク)
  *  https://tauri.app/
  * メインプロセスはRust、UIはOSが提供するWebViewをRustのライブラリがラップしたものを使用する
  * 一般的にはフロントエンドとバックエンドはWebAPIなどの非同期通信で実装されますが、Tauriはその両方がパッケージ化され、内部でプロセス間通信する。
  * UIがOSのWebViewを拡張したRust実装であるため、ブラウザに向けたフロントエンドWEBフレームワークが利用可能。当プロジェクトは **React+TypeScript**を利用。
## 開発環境
* Windows 11
* [VS Code](https://code.visualstudio.com/) 
  * 拡張機能
    *  [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
    * [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
* Rust
  ```
  $ cargo --version
  cargo 1.78.0 (54d8815d0 2024-03-26)

  $ rustc -V
  rustc 1.78.0 (9b00956e5 2024-04-29)

  $ rustup -V
  rustup 1.27.1 (54dd3d00f 2024-04-24)

  $ rustup show
  Default host: x86_64-pc-windows-msvc
  rustup home:  

  stable-x86_64-pc-windows-msvc (default)
  rustc 1.78.0 (9b00956e5 2024-04-29)
  ```
* Tauri
  ```
  $ cargo install create-tauri-app --locked
  ```
* Create Project
  ```
  D:\dev>npm create tauri-app

  > npx
  > create-tauri-app

  ✔ Project name · my_project_name
  ✔ Choose which language to use for your frontend ·   TypeScript / JavaScript - (pnpm, yarn, npm, bun)
  ✔ Choose your package manager · npm
  ✔ Choose your UI template · React - (https://react.dev/)
  ✔ Choose your UI flavor · TypeScript

  Template created! To get started run:
    cd my_project_name
    npm install
    npm run tauri dev
  ```
    * npmでプロジェクト作成する。（Cargoのプロジェクト作成ではReactが選択できなかったのでnpxを使用。謎）
* Project Setting(最低限)
  * tauri.conf.json の以下部分をユニークなものに変更
    * "identifier": "com.tauri.dev",
    * ↓ 例
    * "identifier": "com.genhirano.dev",  など
* dependency Install
  ```
  npm install
  ```
* run Project
  ```
  npm run tauri dev 
  ```
* Build
  ```
  > $ npm run tauri build
  ```
  * インストーラーが作成される（２種類） どちらも同じ。
    * msi
      * \src-tauri\target\release\bundle\msi\***.msi
    * exe
      * \src-tauri\target\release\bundle\nsis\***x64-setup.exe
  * インストールして実行する

## その他
* Rustのエントリーポイントを増やして、Rustだけで実行できるようにする方法
  * Step.1  Cargo.tomlに以下の２つを追加
  ```
  [[bin]]
  name = "default"
  path = "src/main.rs"

  [[bin]]
  name = "main_only_rust"
  path = "src/main_only_rust.rs"
  ```
   * nameはなんでもいい
   * エントリーポイントとなる rs ファイルを pathで指定
  * Step.2　実行
  ```
  cd src-tauri
  cargo run --bin main_only_rust
  ```
   * cargoで実行。--bin で、Cargo.tomlで作ったものを指定する
   * 参考
     * https://paruma184.hatenablog.com/entry/2021/09/22/210945
