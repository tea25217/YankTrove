export interface VideoDownloadTarget {
  id: string;
  url: string;
}

export interface DownloadOptions {
  chat: boolean;
  metadata: boolean;
  description: boolean;
  subtitles: boolean;
  thumbnail: boolean;
  video: boolean;
  audio: boolean;
  audio_format: string;
  cookies_browser: string;
}

export interface VideoInfo {
  id: string;
  title: string;
  url: string;
  duration: number | null;
}

export interface ChannelInfo {
  channel_title: string;
  videos: VideoInfo[];
}

export interface ProgressPayload {
  video_id: string;
  percentage: number;
  speed: string | null;
  eta: string | null;
  status: string;
  log: string | null;
}
