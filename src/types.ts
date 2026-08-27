export interface VideoDownloadTarget {
  id: string;
  url: string;
  title: string;
  duration: number | null;
  uploaded_at: string | null;
  availability: string | null;
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
  video_quality: string;
  cookies_browser: string;
  csv: boolean;
  create_yanktrove_folder: boolean;
  overwrite_mode: 'overwrite' | 'skip' | 'ask';
}

export interface VideoInfo {
  id: string;
  title: string;
  url: string;
  duration: number | null;
  uploaded_at: string | null;
  availability: string | null;
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
