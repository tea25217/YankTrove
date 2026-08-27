# Yank Trove

Yank Trove は、YouTube 向けのチE��クトップアプリケーションです。指定したチャンネルめE�Eレイリストから、メンバ�E限定�E信を含むアーカイブ動画のチE�Eタ�E�動画・音声・チャチE��・メタチE�Eタ・字幕�Eサムネイル等）をまとめて保存できます、E
本アプリは YouTube 公弁EAPI を使わず、[yt-dlp](https://github.com/yt-dlp/yt-dlp) を通じて公開�EージめE��トリーム惁E��を取得します。ブラウザのクチE��ーを使ぁE��合�E、ログイン中のアカウントとしてアクセスします、E
**利用は自己責任です、E* YouTube の利用規紁E��法令に反する使ぁE��、E��度な連続取得、ログイン状態での取得などにより、IP 制限、機�E制限、アカウント停止などの不利益が生じても、E��発老E�E責任を負ぁE��せん。取得してよいコンチE��チE��どぁE���E�著作権・メンバ�EシチE�E契紁E��含む�E�も、利用老E�E身で判断してください、E
**対忁EOS:** 動作確認済みは **Windows** です。macOS 向けのビルド手頁E�E設定�E用意してぁE��すが、E��発老E�� Mac を所持してぁE��ぁE��めE**動作未確誁E* です、Einux�E�E.deb` 等）�E未対応です、E
**配币EID:** `com.yanktrove.desktop`。以前�E `com.yanktrove.app` から変更したため、OS 上では別アプリとして扱われます（既存インスト�Eルの上書き�E設定引き継ぎはありません�E�、E
**表示言誁E** 初回は OS が日本語なら日本語、それ以外�E英語。�EチE��ーで刁E��替えられ、E��択�E次回起動以降も維持されます、E
**設計判断:** 後から要E��とコストが大きい判断は [docs/adr](./docs/adr/) に記録してぁE��す、E
---

## 主な機�E

- **一括チE�Eタ取征E*: メタチE�Eタ (JSON)、チャチE��ログ (JSON)、概要欁E(.txt)、字幁E(.vtt)、サムネイル (.jpg)、動画本佁E(MP4・画質選択可)、E��声 (MP3 / M4A)、整形サマリ (CSV) を個別選択して保孁E- **メンバ�E限定�E信対忁E*: ブラウザ�E�Ehrome / Edge / Firefox / Safari�E��EクチE��ーを読み込んでメン限コンチE��チE��取征E- **チャンネルタブ�E動展開**: チャンネルトッチEURL から Videos / Live / Shorts を展開し、個別動画を一覧表示
- **オリジナル言語字幁E*: 動画の原語（日本語�E信なら日本語、英語�E信なら英語）�E字幕を優先取得。�E動翻訳字幕�E除夁E- **IPブロチE��回避�E�レートリミット対策！E*: 連続ダウンロード時に任意�E征E��時間（秒）を設定可能
- **中断・破棁E���E**: ダウンロードをぁE��でもキャンセルでき、一時ファイル (`.part` / `.ytdl`) を�E動クリーンアチE�E
- **モダンチE��イン**: 見やすいライトテーチEUI。�EチE��ーで JS Runtime / FFmpeg の検�E状態を表示

---



## 忁E��な環墁E


### JS Runtime�E�Eeno�E� EYouTube 向けの取得に忁E��E
YouTube 向けの取得�E琁E��Et-dlp�E�には JavaScript ランタイムが忁E��です、E

| プラチE��フォーム                   | 対忁E                 |
| -------------------------- | ------------------- |
| **Windows�E�インスト�Eラー版！E*      | Deno を同梱。追加インスト�Eル不要E|
| **Windows�E�開発ビルド！E/ macOS** | 以下�EぁE��れかめEPATH に用愁E  |




#### 手動インスト�Eル�E�Eindows / macOS 共通！E
```cmd
winget install DenoLand.Deno
```

macOS の場吁E

```bash
brew install deno
```

インスト�Eル後、アプリを�E起動し、右上バチE��ぁE**「JS Runtime: Deno 検�E済み、E* になることを確認してください、E
参老E [yt-dlp EJS セチE��アチE�Eガイド](https://github.com/yt-dlp/yt-dlp/wiki/EJS)

---



### FFmpeg  E動画・音声を保存するときだけ忁E��E
**FFmpeg** は、動画ファイルの作�EめE��声の変換を行う無料�Eソフトです、Eank Trove 本体とは別に、PC に入れておく忁E��があります、E

| 保存する�E容                  | FFmpeg |
| ----------------------- | ------ |
| 動画�E�EP4�E��E音声�E�EP3 / M4A�E�E  | **忁E��E* |
| 字幕�EメタチE�Eタ・サムネイル・チャチE��ログのみ | **不要E* |


インスト�Eル後、Yank Trove を一度終亁E��て再起動し、画面右上が **「FFmpeg: 検�E済み、E* になれ�E準備完亁E��す、E
#### Windows でのインスト�Eル

1. キーボ�Eド�E **Windows キー**�E�⊞�E�を押し、E*「cmd、E* と入力して **Enter** を押します、E
開いたウィンドウに、以下を**コピ�Eして貼り付け**、E*Enter** を押します、E
```
 winget install Gyan.FFmpeg
```

1. 「同意しますか�E�」などと表示されたら、E*Y** キーを押して **Enter** を押します、E2. 「インスト�Eルが完亁E��ました」と表示されたら、ウィンドウは閉じてかまぁE��せん、E3. **Yank Trove を一度終亁E��、もぁE��度起勁E* してください。右上が **「FFmpeg: 検�E済み、E* になれ�E成功です、E
ぁE��くいかなぁE��合�E、PC めE**再起勁E* してから、手頁E5 をもぁE��度お試しください、E
#### macOS�E�動作未確認！E
```bash
brew install ffmpeg
```

インスト�Eル後、右上バチE��ぁE**「FFmpeg: 検�E済み、E* になることを確認してください、E 
※ macOS は開発環墁E��の動作確認ができてぁE��せん。問題があれば Issue で報告してください、E
---



## 使ぁE��ガイチE
1. **アプリを起勁E*: Yank Trove を起動します、E2. **チャンネル URL の入劁E*:
  - 例（チャンネル全体！E `https://www.youtube.com/@ChannelName`
  - 例！Eive アーカイブ�Eみ�E�E `https://www.youtube.com/@ChannelName/streams`
  - プレイリスチEURL も利用可能です、E3. **クチE��ーの選択（メン限を取得する場合！E*:
  - メンバ�EシチE�E加入済みアカウントで YouTube にログインしたブラウザを選択します、E  - **Firefox を推奨**します。Windows の Chrome / Edge は Cookie をアプリ専用に暗号化するため、Yank Trove から読めなぁE��とがあります！Eyt-dlp#10927](https://github.com/yt-dlp/yt-dlp/issues/10927)�E�、E4. **保存�Eフォルダ**: チE��ォルト�E `ダウンローチEYankTrove/{チャンネル名}/{投稿日晁E_{タイトル}/`�E�投稿日時�E UTC の `YYYYMMDD-hhmm`�E�。「選択」�Eタンで変更可能。`YankTrove` フォルダの作�E有無はチェチE��で刁E��替えられる、E5. **既存ファイル**: 「上書き」「スキチE�E」「毎回選択」から選べる（既定�EスキチE�E�E�、E6. **取得データの選抁E*: 保存したい頁E��にチェチE��を�Eれます。動画本体を選んだとき�E画質�E�最高、E60p�E�を一括持E��でき、その解像度が無ぁE��画はそれ以下�E最良に落ちます（音質は下げません�E�、E7. **動画リストを取征E*: 右ペインに動画一覧が表示されます（件数が多い場合�E時間がかかります）。タイトル・配信日・公開／メンバ�E限定で絞り込めます（リスト取得時点の惁E��を使用。�Eラン別の区別は未対応）、E8. **取得開姁E*: 対象動画を選択し「取得開始」をクリチE��します、E


### 保存ファイル名�E侁E
```
YankTrove/{チャンネル名}/{YYYYMMDD-hhmm}_{タイトル}/
  ├── {タイトル} [{動画ID}].info.json
  ├── {タイトル} [{動画ID}].ja-orig.vtt   ↁE日本語�E信の字幁E  ├── {タイトル} [{動画ID}].mp4
  └── ...
YankTrove/{チャンネル名}/summary.csv   ↁE取得データ選択�E「整形チE�Eタ (CSV)、E```

---



## 既知の不�E吁E


### Chrome / Edge クチE��ー読み取り失敗！Eindows�E�E
「使用するブラウザのクチE��ー」で Google Chrome また�E Microsoft Edge を選択すると、クチE��ー読み取りに失敗することがあります。原因は次の2系統があります、E
- **App-Bound Encryption�E�多い�E�E*: `Failed to decrypt with DPAPI`、Eyt-dlp#10927](https://github.com/yt-dlp/yt-dlp/issues/10927)。ブラウザを終亁E��ても直りません
- **起動中の DB ロチE��**: `Could not copy ... cookie database`、Eyt-dlp#7271](https://github.com/yt-dlp/yt-dlp/issues/7271)

失敗時は処琁E��グとエラー本斁E�� **yt-dlp の原文** を残します、Eirefox の使用を推奨します、E
**暫定回避筁E*

- **Firefox を使用する**  EクチE��ー取得�EめEFirefox に刁E��替える�E�EPAPI 失敗時の実質皁E��対処�E�E- **Chrome / Edge を完�E終亁E��てから再試行すめE*  E`Could not copy` のロチE��が原因の場合�Eみ有効でぁE
---



## トラブルシューチE��ング


| 痁E��                                   | 対処                                        |
| ------------------------------------ | ----------------------------------------- |
| リスト取得失敗！Ehrome クチE��ー / DPAPI�E�E        | Firefox に刁E��替える、Ehrome 終亁E��は直らなぁE��Eyt-dlp#10927](https://github.com/yt-dlp/yt-dlp/issues/10927)�E�E|
| `n challenge solving failed`         | Deno�E�また�E Node.js 22+�E�をインスト�Eルして再起勁E        |
| チャンネル URL で Videos/Live/Shorts の3件のみ | 正常動作です、Eive のみ欲しい場合�E `/streams` URL を直接入劁E|
| 字幕が英語（翻訳�E�になめE                        | v0.2.0 以降�E原語字幕を優先。�E取得してください               |
| 動画めE��声が保存できなぁE/ 右上が「FFmpeg: 未検�E、E     | 上記「FFmpeg」�E手頁E��インスト�Eルし、Yank Trove を�E起勁E    |


---



## ビルドとインスト�Eラー生�E

```bash
# 依存関係�Eインスト�Eル
npm install

# 開発モード起勁Enpm run tauri dev

# 生産用パッケージ�E�Eexe / .msi / .dmg 等）�E生�E
npm run tauri build
```

生�E物出力�E: `src-tauri/target/release/bundle/`

### バ�Eジョン番号

当面は `0.x` のままです！E.0 には上げません�E�。破壊的変更もメジャーではなく�Eイナ�Eを上げます、E
- **マイナ�E**�E�E0.x.0`�E�E 機�E追加、およ�E破壊的変更
- **パッチE*�E�E0.0.x`�E�E 些細な変更

### GitHub Actions によるリリース�E�Eindows / macOS�E�E
`v*` タグめEpush すると、[Release workflow](./.github/workflows/release.yml) ぁEWindows�E�ESIS / MSI�E�と macOS�E�Epple Silicon / Intel の `.dmg`�E�をビルドし、GitHub Release に添付します、E
```bash
# バ�Eジョンを上げたうえで:
git tag v0.10.0
git push origin v0.10.0
```

手動実行�E Actions タブ�E **Release** ↁE**Run workflow** からも可能です。macOS 成果物は動作未確認です、E
### 同梱バイナリ�E�開発老E��け！E
`yt-dlp` と Deno の実行ファイルは Git に含めてぁE��せん�E�EitHub のファイルサイズ制限�Eため�E�、E 
配置方法�E [src-tauri/binaries/README.md](./src-tauri/binaries/README.md) を参照してください、EI では `.github/scripts/download-sidecars.sh` が�E動取得します、E
### シークレチE�� / 個人惁E��スキャン

`push` と pull request で [Betterleaks](https://github.com/betterleaks/betterleaks) が走り、API キー・秘寁E��・設定ファイル冁E�Eメール�E�電話番号などを検知すると失敗します、E
ローカルの `git commit` / `git push` でも止める場合（�E回�Eみ�E�E

```powershell
powershell -File scripts/install-git-hooks.ps1
```

フルルールで見るには [Betterleaks](https://github.com/betterleaks/betterleaks) めEPATH に入れてください。未導�Eでも、秘寁E��めEGitHub ト�Eクンなど確度の高いパターンは hook 側のフォールバックで拒否します、E
---



## ライセンス

- 本アプリケーション: [LICENSE](./LICENSE)
- 第三老E��フトウェア: [THIRD_PARTY_LICENSES.md](./THIRD_PARTY_LICENSES.md)

