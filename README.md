# Yank Trove

Yank Trove は、YouTube 向けのデスクトップアプリケーションです。指定したチャンネルやプレイリストから、メンバー限定配信を含むアーカイブ動画のデータ（動画・音声・チャット・メタデータ・字幕・サムネイル等）をまとめて保存できます。

本アプリは YouTube 公式 API を使わず、[yt-dlp](https://github.com/yt-dlp/yt-dlp) を通じて公開ページやストリーム情報を取得します。ブラウザのクッキーを使う場合は、ログイン中のアカウントとしてアクセスします。

**利用は自己責任です。** YouTube の利用規約や法令に反する使い方、過度な連続取得、ログイン状態での取得などにより、IP 制限、機能制限、アカウント停止などの不利益が生じても、開発者は責任を負いません。取得してよいコンテンツかどうか（著作権・メンバーシップ契約を含む）も、利用者自身で判断してください。

**対応 OS:** 動作確認済みは **Windows** です。macOS 向けのビルド手順・設定は用意していますが、開発者が Mac を所持していないため **動作未確認** です。Linux（`.deb` 等）は未対応です。

**配布 ID:** `com.yanktrove.desktop`。以前の `com.yanktrove.app` から変更したため、OS 上では別アプリとして扱われます（既存インストールの上書き・設定引き継ぎはありません）。

**表示言語:** OS の言語が日本語のときは日本語、それ以外は英語です（アプリ内での切替はありません）。

---

## 主な機能

- **一括データ取得**: メタデータ (JSON)、チャットログ (JSON)、概要欄 (.txt)、字幕 (.vtt)、サムネイル (.jpg)、動画本体 (MP4)、音声 (MP3 / M4A)、整形サマリ (CSV) を個別選択して保存
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


| プラットフォーム                   | 対応                  |
| -------------------------- | ------------------- |
| **Windows（インストーラー版）**      | Deno を同梱。追加インストール不要 |
| **Windows（開発ビルド） / macOS** | 以下のいずれかを PATH に用意   |




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



### FFmpeg — 動画・音声を保存するときだけ必要

**FFmpeg** は、動画ファイルの作成や音声の変換を行う無料のソフトです。Yank Trove 本体とは別に、PC に入れておく必要があります。


| 保存する内容                  | FFmpeg |
| ----------------------- | ------ |
| 動画（MP4）・音声（MP3 / M4A）   | **必要** |
| 字幕・メタデータ・サムネイル・チャットログのみ | **不要** |


インストール後、Yank Trove を一度終了して再起動し、画面右上が **「FFmpeg: 検出済み」** になれば準備完了です。

#### Windows でのインストール

1. キーボードの **Windows キー**（⊞）を押し、**「cmd」** と入力して **Enter** を押します。

開いたウィンドウに、以下を**コピーして貼り付け**、**Enter** を押します。

```
 winget install Gyan.FFmpeg
```

1. 「同意しますか？」などと表示されたら、**Y** キーを押して **Enter** を押します。
2. 「インストールが完了しました」と表示されたら、ウィンドウは閉じてかまいません。
3. **Yank Trove を一度終了し、もう一度起動** してください。右上が **「FFmpeg: 検出済み」** になれば成功です。

うまくいかない場合は、PC を **再起動** してから、手順 5 をもう一度お試しください。

#### macOS（動作未確認）

```bash
brew install ffmpeg
```

インストール後、右上バッジが **「FFmpeg: 検出済み」** になることを確認してください。  
※ macOS は開発環境での動作確認ができていません。問題があれば Issue で報告してください。

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
4. **保存先フォルダ**: デフォルトは `ダウンロード/YankTrove/{チャンネル名}/{タイトル} [{動画ID}]/`。「選択」ボタンで変更可能。
5. **取得データの選択**: 保存したい項目にチェックを入れます。
6. **動画リストを取得**: 右ペインに動画一覧が表示されます（件数が多い場合は時間がかかります）。タイトル・配信日・公開／メンバー限定で絞り込めます（リスト取得時点の情報を使用。プラン別の区別は未対応）。
7. **取得開始**: 対象動画を選択し「取得開始」をクリックします。



### 保存ファイル名の例

```
YankTrove/{チャンネル名}/{タイトル} [{動画ID}]/
  ├── {タイトル} [{動画ID}].info.json
  ├── {タイトル} [{動画ID}].ja-orig.vtt   ← 日本語配信の字幕
  ├── {タイトル} [{動画ID}].mp4
  └── ...
YankTrove/{チャンネル名}/summary.csv   ← 取得データ選択の「整形データ (CSV)」
```

---



## 既知の不具合



### Chrome / Edge クッキー読み取り失敗（Windows）

「使用するブラウザのクッキー」で Google Chrome または Microsoft Edge を選択すると、`Could not copy Chrome cookie database` というエラーで動画リストの取得やダウンロードに失敗することがあります。Chromium 系ブラウザが Cookie データベースを排他ロックするためです（[yt-dlp#7271](https://github.com/yt-dlp/yt-dlp/issues/7271)）。アプリ内でも、Chrome / Edge 選択時に同じ案内を表示します。

**暫定回避策**

- **Firefox を使用する** — クッキー取得元を Firefox に切り替える
- **Chrome を完全終了してから再試行する** — タスクマネージャーで Chrome 関連プロセスが残っていないことを確認してから、再度お試しください

---



## トラブルシューティング


| 症状                                   | 対処                                        |
| ------------------------------------ | ----------------------------------------- |
| リスト取得失敗（Chrome クッキー）                 | Chrome を完全終了（タスクマネージャー含む）するか、Firefox を使用  |
| `n challenge solving failed`         | Deno（または Node.js 22+）をインストールして再起動         |
| チャンネル URL で Videos/Live/Shorts の3件のみ | 正常動作です。Live のみ欲しい場合は `/streams` URL を直接入力 |
| 字幕が英語（翻訳）になる                         | v0.2.0 以降は原語字幕を優先。再取得してください               |
| 動画や音声が保存できない / 右上が「FFmpeg: 未検出」      | 上記「FFmpeg」の手順でインストールし、Yank Trove を再起動     |


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

### GitHub Actions によるリリース（Windows / macOS）

`v*` タグを push すると、[Release workflow](./.github/workflows/release.yml) が Windows（NSIS / MSI）と macOS（Apple Silicon / Intel の `.dmg`）をビルドし、GitHub Release に添付します。

```bash
# バージョンを上げたうえで:
git tag v0.5.0
git push origin v0.5.0
```

手動実行は Actions タブの **Release** → **Run workflow** からも可能です。macOS 成果物は動作未確認です。

### 同梱バイナリ（開発者向け）

`yt-dlp` と Deno の実行ファイルは Git に含めていません（GitHub のファイルサイズ制限のため）。  
配置方法は [src-tauri/binaries/README.md](./src-tauri/binaries/README.md) を参照してください。CI では `.github/scripts/download-sidecars.sh` が自動取得します。

---



## ライセンス

- 本アプリケーション: [LICENSE](./LICENSE)
- 第三者ソフトウェア: [THIRD_PARTY_LICENSES.md](./THIRD_PARTY_LICENSES.md)

