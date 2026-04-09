# Shift Manager (Tauri)

正社員の半休取得を平等に割り当てることを目的としたネイティブアプリ版です。

## 開発

```sh
npm install
cargo tauri dev
```

## ビルド

```sh
cargo tauri build
```

## ディレクトリ構成

- src
  - フロントエンド（Gleam + Lustre）
- main.js
  - Vite エントリポイント
- ffi.js
  - Tauri 呼び出し境界
- src-tauri
  - Tauri（Rust）
- work_shift_dayoff_logic
  - シフト割当ロジック（submodule）
