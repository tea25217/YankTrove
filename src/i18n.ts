export type Locale = 'ja' | 'en';

const ja = {
  urlLabel: 'YouTube チャンネル / プレイリスト URL',
  cookieLabel: '使用するブラウザのクッキー',
  cookieNone: '使用しない (公開動画のみ)',
  cookieFirefox: 'Firefox（推奨）',
  cookieChrome: 'Google Chrome',
  cookieEdge: 'Microsoft Edge',
  cookieSafari: 'Safari',
  cookieLockWarning:
    '⚠️ Chrome / Edge が起動中だと、クッキーを読み取れずリスト取得やダウンロードに失敗することがあります。Firefox を使うか、対象ブラウザを完全終了してから再試行してください。',
  saveDirLabel: '保存先フォルダ',
  saveDirPlaceholder: 'デフォルト: ダウンロード/YankTrove',
  browseDir: '選択',
  dataSelect: '取得データ選択',
  optMetadata: 'メタデータ (JSON形式)',
  optChat: 'チャットログ (JSON形式)',
  optDescription: '概要欄テキスト (.txt)',
  optSubtitles: '字幕ファイル (.vtt)',
  optThumbnail: 'サムネイル画像 (JPG)',
  optVideo: '動画本体 (MP4)',
  optAudio: '音声のみ抽出 (MP3)',
  optCsv: '整形データ (CSV)',
  ffmpegWarning:
    '⚠️ 動画の保存、または音声の抽出には <strong>FFmpeg</strong> が必要です。未検出の場合は処理が失敗するか画質が制限される場合があります。',
  audioFormat: '形式:',
  delayLabel: '連続取得時の待機時間 (秒) - IPブロック対策',
  fetchList: '動画リストを取得',
  fetchingList: 'リスト取得中...',
  queueTitle: '動画キュー',
  queueStats: '選択: {selected} / {total} 件',
  queueEmpty: 'URLを入力し、「動画リストを取得」をクリックしてください。',
  start: '取得開始',
  cancel: 'キャンセル',
  cancelling: 'キャンセル中...',
  processingVideo: '動画処理中...',
  etaRemaining: '残り: {eta}',
  etaIdle: '残り: -',
  logTitle: '処理ログ',
  statusIdle: '待機中',
  statusFetching: '取得中',
  statusDownloading: 'ダウンロード中',
  statusDone: '完了',
  statusCancelled: 'キャンセル済み',
  statusError: 'エラー',
  startedLog: 'システムを起動しました。',
  jsRuntimeChecking: 'JS Runtime: 検証中...',
  jsRuntimeFound: 'JS Runtime: {name} 検出済み',
  jsRuntimeMissing: 'JS Runtime: 未検出',
  jsRuntimeFoundLog: '{name} の検出に成功しました。',
  jsRuntimeMissingLog:
    'JavaScript ランタイム（Deno 推奨）が検出されませんでした。YouTube 取得が失敗する可能性があります。`winget install DenoLand.Deno` でインストールできます。',
  ffmpegChecking: 'FFmpeg: 検証中...',
  ffmpegFound: 'FFmpeg: 検出済み',
  ffmpegMissing: 'FFmpeg: 未検出',
  ffmpegFoundLog: 'FFmpeg の検出に成功しました。',
  ffmpegMissingLog:
    'FFmpeg がシステム上に検出されませんでした。動画の結合や音声抽出が失敗する可能性があります。',
  envCheckError: '環境チェック中にエラーが発生しました: {error}',
  browseDirTitle: '保存先フォルダの選択',
  dirChanged: '保存先フォルダを変更しました: {path}',
  browseDirError: 'フォルダ選択中にエラーが発生しました: {error}',
  urlRequired: 'エラー: チャンネルまたはプレイリストのURLを入力してください。',
  fetchingListLog: '動画リストを取得しています: {url}',
  cookieLockHint:
    'Chrome / Edge が起動中だとクッキー読み取りに失敗することがあります。失敗したら Firefox を使うか、ブラウザを完全終了して再試行してください。',
  parsingVideos: '動画情報を解析中...',
  queueTitleWithChannel: '{channel} - アーカイブキュー',
  noArchives: 'アーカイブ動画が見つかりませんでした。公開設定やクッキー設定を確認してください。',
  listEmpty: '動画リストは空でした。',
  listSuccess: 'リストの取得に成功しました。動画数: {count}',
  listFailed: 'リストの取得に失敗しました。',
  listFailedLog: 'リストの取得に失敗しました: {error}',
  selectAll: 'すべて選択/解除',
  statusWaiting: '待機中',
  statusWorking: '処理中...',
  statusComplete: '完了',
  noVideosSelected: 'エラー: キューから取得対象の動画を1つ以上選択してください。',
  downloadStarted: 'ダウンロード処理を開始します。総対象数: {count}',
  cookieLockHintDownload:
    'Chrome / Edge のクッキーを使用します。失敗した場合は Firefox に切り替えるか、ブラウザを完全終了して再試行してください。',
  allDone: 'すべての動画処理が完了しました。',
  downloadCancelled: 'ダウンロードがユーザーによってキャンセルされました。',
  downloadError: 'ダウンロードプロセス中にエラーが発生しました: {error}',
  cancelRequested: 'ダウンロードの中断を要求中...',
  cancelError: 'ダウンロードキャンセル中にエラーが発生しました: {error}',
  downloadStartLog: '動画のダウンロードを開始しました: {title}',
  downloadFinishLog: '動画のダウンロードが完了しました: {title}',
} as const;

const en: { [K in keyof typeof ja]: string } = {
  urlLabel: 'YouTube channel / playlist URL',
  cookieLabel: 'Browser cookies',
  cookieNone: 'Do not use (public videos only)',
  cookieFirefox: 'Firefox (recommended)',
  cookieChrome: 'Google Chrome',
  cookieEdge: 'Microsoft Edge',
  cookieSafari: 'Safari',
  cookieLockWarning:
    '⚠️ If Chrome / Edge is running, cookie read may fail and list/download can abort. Use Firefox, or fully quit the browser and retry.',
  saveDirLabel: 'Save folder',
  saveDirPlaceholder: 'Default: Downloads/YankTrove',
  browseDir: 'Browse',
  dataSelect: 'Data to save',
  optMetadata: 'Metadata (JSON)',
  optChat: 'Chat log (JSON)',
  optDescription: 'Description (.txt)',
  optSubtitles: 'Subtitles (.vtt)',
  optThumbnail: 'Thumbnail (JPG)',
  optVideo: 'Video (MP4)',
  optAudio: 'Audio only (MP3)',
  optCsv: 'Summary spreadsheet (CSV)',
  ffmpegWarning:
    '⚠️ Saving video or extracting audio requires <strong>FFmpeg</strong>. Without it, the job may fail or quality may be limited.',
  audioFormat: 'Format:',
  delayLabel: 'Delay between downloads (seconds) — rate-limit protection',
  fetchList: 'Fetch video list',
  fetchingList: 'Fetching list...',
  queueTitle: 'Video queue',
  queueStats: 'Selected: {selected} / {total}',
  queueEmpty: 'Enter a URL and click “Fetch video list”.',
  start: 'Start',
  cancel: 'Cancel',
  cancelling: 'Cancelling...',
  processingVideo: 'Processing video...',
  etaRemaining: 'ETA: {eta}',
  etaIdle: 'ETA: -',
  logTitle: 'Log',
  statusIdle: 'Idle',
  statusFetching: 'Fetching',
  statusDownloading: 'Downloading',
  statusDone: 'Done',
  statusCancelled: 'Cancelled',
  statusError: 'Error',
  startedLog: 'Application started.',
  jsRuntimeChecking: 'JS Runtime: checking...',
  jsRuntimeFound: 'JS Runtime: {name} found',
  jsRuntimeMissing: 'JS Runtime: not found',
  jsRuntimeFoundLog: 'Detected {name}.',
  jsRuntimeMissingLog:
    'No JavaScript runtime (Deno recommended) was found. YouTube fetches may fail. Install with `winget install DenoLand.Deno`.',
  ffmpegChecking: 'FFmpeg: checking...',
  ffmpegFound: 'FFmpeg: found',
  ffmpegMissing: 'FFmpeg: not found',
  ffmpegFoundLog: 'Detected FFmpeg.',
  ffmpegMissingLog:
    'FFmpeg was not found on this system. Merging video or extracting audio may fail.',
  envCheckError: 'Environment check failed: {error}',
  browseDirTitle: 'Choose save folder',
  dirChanged: 'Save folder changed: {path}',
  browseDirError: 'Folder selection failed: {error}',
  urlRequired: 'Error: enter a channel or playlist URL.',
  fetchingListLog: 'Fetching video list: {url}',
  cookieLockHint:
    'Cookie read can fail while Chrome / Edge is running. If it fails, switch to Firefox or fully quit the browser and retry.',
  parsingVideos: 'Parsing video information...',
  queueTitleWithChannel: '{channel} — archive queue',
  noArchives: 'No archive videos found. Check visibility and cookie settings.',
  listEmpty: 'The video list was empty.',
  listSuccess: 'List fetched. Videos: {count}',
  listFailed: 'Failed to fetch the list.',
  listFailedLog: 'Failed to fetch the list: {error}',
  selectAll: 'Select / deselect all',
  statusWaiting: 'Waiting',
  statusWorking: 'Working...',
  statusComplete: 'Done',
  noVideosSelected: 'Error: select at least one video in the queue.',
  downloadStarted: 'Starting download. Total: {count}',
  cookieLockHintDownload:
    'Using Chrome / Edge cookies. If this fails, switch to Firefox or fully quit the browser and retry.',
  allDone: 'Finished processing all videos.',
  downloadCancelled: 'Download was cancelled.',
  downloadError: 'Download failed: {error}',
  cancelRequested: 'Requesting download cancellation...',
  cancelError: 'Failed to cancel download: {error}',
  downloadStartLog: 'Started download: {title}',
  downloadFinishLog: 'Finished download: {title}',
};

const catalogs = { ja, en };

let currentLocale: Locale = 'ja';

export function detectLocale(language = navigator.language): Locale {
  return language.toLowerCase().startsWith('ja') ? 'ja' : 'en';
}

export function initI18n(): Locale {
  currentLocale = detectLocale();
  document.documentElement.lang = currentLocale;
  return currentLocale;
}

export function locale(): Locale {
  return currentLocale;
}

export type MessageKey = keyof typeof ja;

export function t(key: MessageKey, vars?: Record<string, string | number>): string {
  let text: string = catalogs[currentLocale][key];
  if (vars) {
    for (const [name, value] of Object.entries(vars)) {
      text = text.split(`{${name}}`).join(String(value));
    }
  }
  return text;
}
