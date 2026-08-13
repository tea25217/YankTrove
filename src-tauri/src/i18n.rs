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

pub fn cookie_extraction_error_message(browser: &str, locale: UiLocale) -> String {
    let browser_name = match browser {
        "chrome" => "Google Chrome",
        "edge" => "Microsoft Edge",
        "firefox" => "Firefox",
        "safari" => "Safari",
        _ => browser,
    };

    if browser == "chrome" || browser == "edge" {
        return if locale.is_ja() {
            format!(
                "ブラウザ（{}）のクッキーを読み取れませんでした。起動中だと Cookie データベースがロックされます。\n\n\
                次のいずれかを試してください:\n\
                1. Firefox で YouTube にログインし、「使用するブラウザのクッキー」を Firefox に切り替える（推奨）\n\
                2. {} を完全に終了してから再試行する（タスクマネージャーでバックグラウンドプロセスも終了）\n\n\
                参考: https://github.com/yt-dlp/yt-dlp/issues/7271",
                browser_name, browser_name
            )
        } else {
            format!(
                "Could not read cookies from {browser_name}. Chromium locks the cookie database while it is running.\n\n\
                Try one of the following:\n\
                1. Sign in to YouTube in Firefox and select Firefox as the cookie source (recommended)\n\
                2. Fully quit {browser_name} (including background processes in Task Manager) and retry\n\n\
                See: https://github.com/yt-dlp/yt-dlp/issues/7271"
            )
        };
    }

    if locale.is_ja() {
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
    }
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
