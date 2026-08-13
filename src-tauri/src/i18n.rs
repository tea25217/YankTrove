#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiLocale {
    Ja,
    En,
}

impl UiLocale {
    pub fn parse(value: &str) -> Self {
        if value.to_ascii_lowercase().starts_with("ja") {
            Self::Ja
        } else {
            Self::En
        }
    }

    pub fn is_ja(self) -> bool {
        matches!(self, Self::Ja)
    }
}

pub fn cookie_extraction_error_message(browser: &str, locale: UiLocale, ytdlp_detail: &str) -> String {
    let browser_name = match browser {
        "chrome" => "Google Chrome",
        "edge" => "Microsoft Edge",
        "firefox" => "Firefox",
        "safari" => "Safari",
        _ => browser,
    };
    let detail = truncate_ytdlp_detail(ytdlp_detail);

    let body = if browser == "chrome" || browser == "edge" {
        if locale.is_ja() {
            format!(
                "ブラウザ（{}）のクッキーを読み取れませんでした。\n\
                起動中のロック以外に、新しい Chrome / Edge では終了後も外部プログラムが Cookie を復号できないことがあります。\n\n\
                次のいずれかを試してください:\n\
                1. Firefox で YouTube にログインし、「使用するブラウザのクッキー」を Firefox に切り替える（推奨）\n\
                2. {} を完全に終了してから再試行する（タスクマネージャーでバックグラウンドプロセスも終了）\n\n\
                参考: https://github.com/yt-dlp/yt-dlp/issues/7271",
                browser_name, browser_name
            )
        } else {
            format!(
                "Could not read cookies from {browser_name}.\n\
                Besides a lock while the browser is running, newer Chrome / Edge may block decryption even after quit.\n\n\
                Try one of the following:\n\
                1. Sign in to YouTube in Firefox and select Firefox as the cookie source (recommended)\n\
                2. Fully quit {browser_name} (including background processes in Task Manager) and retry\n\n\
                See: https://github.com/yt-dlp/yt-dlp/issues/7271"
            )
        }
    } else if locale.is_ja() {
        format!(
            "ブラウザ（{}）のクッキーを読み取れませんでした。\n\n\
            メン限動画を取得するには、以下のいずれかをお試しください:\n\
            1. {} を完全に終了してから再試行（タスクマネージャーでバックグラウンドプロセスも終了）\n\
            2. 別のブラウザ（Firefox 等）で YouTube にログインし、そのブラウザを選択\n\n\
            参考: https://github.com/yt-dlp/yt-dlp/issues/7271",
            browser_name, browser_name
        )
    } else {
        format!(
            "Could not read cookies from {browser_name}.\n\n\
            To fetch membership videos, try one of the following:\n\
            1. Fully quit {browser_name} (including background processes) and retry\n\
            2. Sign in to YouTube in another browser (Firefox recommended) and select that browser\n\n\
            See: https://github.com/yt-dlp/yt-dlp/issues/7271"
        )
    };

    if detail.is_empty() {
        body
    } else if locale.is_ja() {
        format!("{body}\n\nyt-dlp 原文:\n{detail}")
    } else {
        format!("{body}\n\nyt-dlp output:\n{detail}")
    }
}

fn truncate_ytdlp_detail(detail: &str) -> String {
    let lines: Vec<&str> = detail
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();
    if lines.is_empty() {
        return String::new();
    }
    let start = lines.len().saturating_sub(12);
    lines[start..].join("\n")
}

pub fn js_runtime_setup_message(locale: UiLocale) -> String {
    if locale.is_ja() {
        "YouTube 取得には JavaScript ランタイム（Deno 推奨）が必要です。\n\n\
        以下のいずれかで Deno をインストールしてください:\n\
        1. winget install DenoLand.Deno\n\
        2. https://docs.deno.com/runtime/getting_started/installation/\n\n\
        インストール後、アプリを再起動してください。\n\
        参考: https://github.com/yt-dlp/yt-dlp/wiki/EJS"
            .to_string()
    } else {
        "Fetching from YouTube requires a JavaScript runtime (Deno recommended).\n\n\
        Install Deno with one of:\n\
        1. winget install DenoLand.Deno\n\
        2. https://docs.deno.com/runtime/getting_started/installation/\n\n\
        Restart the app after installing.\n\
        See: https://github.com/yt-dlp/yt-dlp/wiki/EJS"
            .to_string()
    }
}

pub fn channel_not_found_error(url: &str, err_summary: &str, locale: UiLocale) -> String {
    let detail = err_summary.lines().last().unwrap_or("");
    if locale.is_ja() {
        format!(
            "チャンネルが見つかりませんでした。\nURLを確認してください: {}\n詳細: {}",
            url, detail
        )
    } else {
        format!(
            "Channel not found.\nCheck the URL: {}\nDetails: {}",
            url, detail
        )
    }
}

pub fn ytdlp_fetch_error(err_summary: &str, locale: UiLocale) -> String {
    if locale.is_ja() {
        format!("yt-dlp 取得エラー:\n{}", err_summary)
    } else {
        format!("yt-dlp fetch error:\n{}", err_summary)
    }
}

pub fn video_list_unavailable(err_summary: &str, locale: UiLocale) -> String {
    if locale.is_ja() {
        format!(
            "動画リストを取得できませんでした。URLまたはクッキー設定を確認してください。\n詳細:\n{}",
            err_summary
        )
    } else {
        format!(
            "Could not fetch the video list. Check the URL or cookie settings.\nDetails:\n{}",
            err_summary
        )
    }
}

pub fn waiting_before_next(delay_seconds: u64, locale: UiLocale) -> String {
    if locale.is_ja() {
        format!("次の動画まで {} 秒待機します...", delay_seconds)
    } else {
        format!("Waiting {delay_seconds} seconds before the next video...")
    }
}

pub fn download_cancelled_log(video_id: &str, locale: UiLocale) -> String {
    if locale.is_ja() {
        format!("ダウンロードをキャンセルし、一時ファイルを削除しました。ID: {}", video_id)
    } else {
        format!("Download cancelled and temporary files cleaned up. ID: {video_id}")
    }
}

pub fn video_failed_log(video_id: &str, error: &str, locale: UiLocale) -> String {
    if locale.is_ja() {
        format!("[ERROR] 動画 ID {} が失敗しました: {}", video_id, error)
    } else {
        format!("[ERROR] Video ID {video_id} failed: {error}")
    }
}

pub fn csv_header(locale: UiLocale) -> [&'static str; 7] {
    if locale.is_ja() {
        ["動画ID", "タイトル", "URL", "配信日時", "長さ", "公開状態", "保存パス"]
    } else {
        ["video_id", "title", "url", "uploaded_at", "duration", "availability", "save_path"]
    }
}

pub fn csv_availability_label(raw: &str, locale: UiLocale) -> String {
    let key = raw.split('/').next().unwrap_or(raw).trim();
    let mapped = match key {
        "public" => if locale.is_ja() { "公開" } else { "Public" },
        "unlisted" => if locale.is_ja() { "限定公開" } else { "Unlisted" },
        "private" => if locale.is_ja() { "非公開" } else { "Private" },
        "subscriber_only" | "premium_only" => {
            if locale.is_ja() { "メンバー限定" } else { "Members only" }
        }
        "needs_auth" => if locale.is_ja() { "要ログイン" } else { "Login required" },
        "" => "",
        _ => key,
    };

    if mapped.is_empty() {
        raw.to_string()
    } else if raw.contains('/') {
        format!("{mapped} ({raw})")
    } else if mapped == key {
        raw.to_string()
    } else {
        mapped.to_string()
    }
}

pub fn csv_written_log(path: &str, locale: UiLocale) -> String {
    if locale.is_ja() {
        format!("サマリ CSV を保存しました: {}", path)
    } else {
        format!("Saved summary CSV: {path}")
    }
}

pub fn csv_write_failed_log(error: &str, locale: UiLocale) -> String {
    if locale.is_ja() {
        format!("サマリ CSV の保存に失敗しました: {}", error)
    } else {
        format!("Failed to save summary CSV: {error}")
    }
}
