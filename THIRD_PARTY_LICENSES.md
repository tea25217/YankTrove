# 📜 第三者ソフトウェア・ライブラリ ライセンス一覧 (Third-Party Licenses)

本アプリケーション（Yank Trove）は、以下のオープンソースソフトウェア（OSS）およびライブラリを利用して開発されています。

---

## 1. 外部プロセス / 連携ツール

### 🔹 yt-dlp
- **用途**: YouTube動画およびメタデータ、チャットログ、字幕の解析・取得
- **ライセンス**: [Unlicense](https://unlicense.org/) (Public Domain / CC0 1.0)
- **公式サイト**: https://github.com/yt-dlp/yt-dlp
- **概要**: 商業利用・再配布問わず極めて自由な利用が許可されています。
- **同梱物**: 本アプリの Windows 版インストーラーには `yt-dlp` 実行ファイルを同梱しています（macOS 版も同様）。

### 🔹 yt-dlp-ejs（yt-dlp 同梱）
- **用途**: YouTube の JavaScript チャレンジ（n-challenge 等）の解決
- **ライセンス**: 公開ドメイン（Unlicense）— yt-dlp 公式 PyInstaller ビルドに同梱
- **公式サイト**: https://github.com/yt-dlp/ejs
- **概要**: yt-dlp 実行ファイル内に含まれるスクリプト群です。単体での再配布は行っていません。

### 🔹 Deno
- **用途**: yt-dlp が YouTube 取得時に利用する JavaScript ランタイム
- **ライセンス**: [MIT License](https://github.com/denoland/deno/blob/main/LICENSE.md)
- **公式サイト**: https://deno.com/
- **概要**:
  - Windows 版インストーラーには Deno 実行ファイルを同梱しています。
  - macOS 版では Homebrew 等による別途インストール、または PATH 上の Deno / Node.js（v22+）を利用します。
  - 本アプリケーションは Deno を外部プロセスとして呼び出すのみで、ソースコードへの静的リンクは行っていません。

### 🔹 FFmpeg
- **用途**: 動画フォーマット変換、音声抽出、マルチメディア結合処理
- **ライセンス**: [LGPL v2.1+](https://www.gnu.org/licenses/old-licenses/lgpl-2.1.html)（一部ビルドオプションにより GPL）
- **公式サイト**: https://ffmpeg.org/
- **遵守事項**:
  - 本アプリケーションは FFmpeg を静的リンクせず、外部プロセス（コマンドラインツール）として独立して呼び出しています。
  - これにより、LGPL / GPL の動的結合要件を満たしており、本アプリケーション自体のソースコードを開示・GPL化する義務は生じません。
  - FFmpeg のソースコードは上記公式サイトより自由に入手可能です。
- **概要**: ユーザー環境への別途インストールが必要です（本アプリには同梱しません）。

---

## 2. フレームワークおよびコアライブラリ

### 🔹 Tauri Framework（Tauri Core, Tauri CLI, Plugins）
- **ライセンス**: [MIT License](https://opensource.org/licenses/MIT) または [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0)
- **公式サイト**: https://tauri.app/

### 🔹 Rust エコシステム（`tokio`, `serde`, `serde_json`, `regex`, `anyhow`, `thiserror`）
- **ライセンス**: MIT License または Apache License 2.0

### 🔹 フロントエンド エコシステム（`Vite`, `TypeScript`）
- **ライセンス**: MIT License
- **公式サイト**: https://vitejs.dev/ / https://www.typescriptlang.org/

---

## 3. オープンソースライセンス全文（標準 MIT License 例）

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
