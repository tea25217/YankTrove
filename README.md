# Yank Trove

Yank Trove は、YouTube 向けのデスクトップアプリケーションです。指定したチャンネルやプレイリストから、メンバー限定配信を含むアーカイブ動画のデータ（動画・音声・チャット・メタデータ・字幕・サムネイル等）をまとめて保存できます。

本アプリは YouTube 公式 API を使わず、[yt-dlp](https://github.com/yt-dlp/yt-dlp) を通じて公開ページやストリーム情報を取得します。ブラウザのクッキーを使う場合は、ログイン中のアカウントとしてアクセスします。

**利用は自己責任です。** YouTube の利用規約や法令に反する使い方、過度な連続取得、ログイン状態での取得などにより、IP 制限、機能制限、アカウント停止などの不利益が生じても、開発者は責任を負いません。取得してよいコンテンツかどうか（著作権・メンバーシップ契約を含む）も、利用者自身で判断してください。

---

## 主な機能

- **一括データ取得**: メタデータ (JSON)、チャットログ (JSON)、概要欄 (.txt)、字幕 (.vtt)、サムネイル (.jpg)、動画本体 (MP4)、音声 (MP3 / M4A) を個別選択して保存
- **メンバー限定配信対応**: ブラウザ（Chrome / Edge / Firefox / Safari）のクッキーを読み込んでメン限コンテンツを取得
- **チャンネルタブ自動展開**: チャンネルトップ URL から Videos / Live / Shorts を展開し、個別動画を一覧表示
- **オリジナル言語字幕**: 動画の原語（日本語配信なら日本語、英語配信なら英語）の字幕を優先取得。自動翻訳字幕は除外
- **IPブロック回避（レートリミット対策）**: 連続ダウンロード時に任意の待機時間（秒）を設定可能
- **中断・破棄機能**: ダウンロードをいつでもキャンセルでき、一時ファイル (`.part` / `.ytdl`) を自動クリーンアップ
- **モダンデザイン**: 見やすいライトテーマ UI。ヘッダーで JS Runtime / FFmpeg の検出状態を表示

---

## 必要な環境

### JS Runtime（Deno）— YouTube 向けの取得に必須

YouTube 向けの取得処理（yt-dlp）には JavaScript ランタイムが必要です。

| プラットフォーム | 対応 |
|-----------------|------|
| **Windows（インストーラー版）** | Deno を同梱。追加インストール不要 |
| **Windows（開発ビルド） / macOS** | 以下のいずれかを PATH に用意 |

#### 手動インストール（Windows / macOS 共通）

```cmd
winget install DenoLand.Deno
```

macOS の場合:

```bash
brew install deno
```

インストール後、アプリを再起動し、右上バッジが **「JS Runtime: Deno 検出済み」** になることを確認してください。

参考: [yt-dlp EJS セットアップガイド](https://github.com/yt-dlp/yt-dlp/wiki/EJS)

---

### FFmpeg — 動画・音声取得時に必須

動画の結合（最高画質 MP4）や音声抽出（MP3 / M4A）には **FFmpeg** が必要です。メタデータ・字幕のみの取得では不要です。

#### Windows

```cmd
winget install Gyan.FFmpeg
```

または [FFmpeg Build (Gyan.dev)](https://www.gyan.dev/ffmpeg/builds/) からダウンロードし、`ffmpeg.exe` のパスを PATH に追加します。

#### macOS

```bash
brew install ffmpeg
```

インストール後、右上バッジが **「FFmpeg: 検出済み」** になることを確認してください。

---

## 使い方ガイド

1. **アプリを起動**: Yank Trove を起動します。
2. **チャンネル URL の入力**:
   - 例（チャンネル全体）: `https://www.youtube.com/@ChannelName`
   - 例（Live アーカイブのみ）: `https://www.youtube.com/@ChannelName/streams`
   - プレイリスト URL も利用可能です。
3. **クッキーの選択（メン限を取得する場合）**:
   - メンバーシップ加入済みアカウントで YouTube にログインしたブラウザを選択します。
   - **Firefox を推奨**します。Chrome / Edge は起動中だとクッキー読み取りに失敗することがあります（完全終了してから再試行するか、Firefox を使用してください）。
4. **保存先フォルダ**: デフォルトは `ダウンロード/YankTrove/{チャンネル名}/`。「選択」ボタンで変更可能。
5. **取得データの選択**: 保存したい項目にチェックを入れます。
6. **動画リストを取得**: 右ペインに動画一覧が表示されます（件数が多い場合は時間がかかります）。
7. **取得開始**: 対象動画を選択し「取得開始」をクリックします。

### 保存ファイル名の例

```
{タイトル} [{動画ID}].info.json
{タイトル} [{動画ID}].ja-orig.vtt   ← 日本語配信の字幕
{タイトル} [{動画ID}].mp4
```

---

## トラブルシューティング

| 症状 | 対処 |
|------|------|
| リスト取得失敗（Chrome クッキー） | Chrome を完全終了（タスクマネージャー含む）するか、Firefox を使用 |
| `n challenge solving failed` | Deno（または Node.js 22+）をインストールして再起動 |
| チャンネル URL で Videos/Live/Shorts の3件のみ | 正常動作です。Live のみ欲しい場合は `/streams` URL を直接入力 |
| 字幕が英語（翻訳）になる | v0.2.0 以降は原語字幕を優先。再取得してください |
| 動画結合・音声抽出失敗 | FFmpeg をインストールして再起動 |

---

## ビルドとインストーラー生成

```bash
# 依存関係のインストール
npm install

# 開発モード起動
npm run tauri dev

# 生産用パッケージ（.exe / .msi / .dmg 等）の生成
npm run tauri build
```

生成物出力先: `src-tauri/target/release/bundle/`

### 同梱バイナリ（開発者向け）

`yt-dlp` と Deno の実行ファイルは Git に含めていません（GitHub のファイルサイズ制限のため）。  
配置方法は [src-tauri/binaries/README.md](./src-tauri/binaries/README.md) を参照してください。

---

## ライセンス

- 本アプリケーション: [LICENSE](./LICENSE)
- 第三者ソフトウェア: [THIRD_PARTY_LICENSES.md](./THIRD_PARTY_LICENSES.md)
