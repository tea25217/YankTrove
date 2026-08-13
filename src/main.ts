import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { open } from '@tauri-apps/plugin-dialog';
import { VideoInfo, ChannelInfo, ProgressPayload } from './types';
import { initI18n, locale, t } from './i18n';

// State management variables
let fetchedVideos: VideoInfo[] = [];
let selectedDir: string | null = null;
let currentChannelTitle = "Channel";
let isDownloading = false;

document.addEventListener('DOMContentLoaded', async () => {
  const appEl = document.getElementById('app');
  if (!appEl) return;

  initI18n();

  // Initialize UI structure
  appEl.innerHTML = `
    <header>
      <div class="logo-section">
        <h1><span class="logo-dot"></span>Yank Trove</h1>
      </div>
      <div class="status-badges">
        <div id="js-runtime-status" class="status-badge warn">${t('jsRuntimeChecking')}</div>
        <div id="ffmpeg-status" class="status-badge warn">${t('ffmpegChecking')}</div>
      </div>
    </header>
    <div class="main-container">
      <div class="left-pane">
        <div class="form-group">
          <label for="channel-url">${t('urlLabel')}</label>
          <input type="text" id="channel-url" placeholder="https://www.youtube.com/..." />
        </div>
        
        <div class="form-group">
          <label for="cookies-browser">${t('cookieLabel')}</label>
          <select id="cookies-browser">
            <option value="none">${t('cookieNone')}</option>
            <option value="firefox">${t('cookieFirefox')}</option>
            <option value="chrome">${t('cookieChrome')}</option>
            <option value="edge">${t('cookieEdge')}</option>
            <option value="safari">${t('cookieSafari')}</option>
          </select>
          <div id="cookie-lock-warning" class="warning-box">
            ${t('cookieLockWarning')}
          </div>
        </div>

        <div class="form-group">
          <label>${t('saveDirLabel')}</label>
          <div class="input-wrapper">
            <input type="text" id="download-dir" readonly placeholder="${t('saveDirPlaceholder')}" />
            <button type="button" id="browse-dir" class="browse-btn">${t('browseDir')}</button>
          </div>
        </div>

        <div class="checkbox-card">
          <h3>${t('dataSelect')}</h3>
          <label class="checkbox-option">
            <input type="checkbox" id="opt-metadata" checked /> ${t('optMetadata')}
          </label>
          <label class="checkbox-option">
            <input type="checkbox" id="opt-chat" checked /> ${t('optChat')}
          </label>
          <label class="checkbox-option">
            <input type="checkbox" id="opt-description" checked /> ${t('optDescription')}
          </label>
          <label class="checkbox-option">
            <input type="checkbox" id="opt-subtitles" checked /> ${t('optSubtitles')}
          </label>
          <label class="checkbox-option">
            <input type="checkbox" id="opt-thumbnail" checked /> ${t('optThumbnail')}
          </label>
          <label class="checkbox-option">
            <input type="checkbox" id="opt-video" /> ${t('optVideo')}
          </label>
          <label class="checkbox-option">
            <input type="checkbox" id="opt-audio" /> ${t('optAudio')}
          </label>
          <div id="ffmpeg-warning" class="warning-box">
            ${t('ffmpegWarning')}
          </div>
          <div id="audio-format-group" class="audio-format-group">
            <label for="audio-format">${t('audioFormat')}</label>
            <select id="audio-format">
              <option value="mp3">MP3</option>
              <option value="m4a">M4A</option>
            </select>
          </div>
        </div>

        <div class="form-group">
          <label for="delay-seconds">${t('delayLabel')}</label>
          <input type="number" id="delay-seconds" min="0" max="60" value="5" style="padding: 10px 14px; border-radius: 8px; border: 1px solid var(--border-color); outline: none;" />
        </div>

        <button type="button" id="fetch-list-btn" class="btn btn-secondary">${t('fetchList')}</button>
      </div>
      
      <div class="right-pane">
        <div class="queue-card">
          <div class="queue-header">
            <span class="queue-title" id="channel-title-display">${t('queueTitle')}</span>
            <span class="queue-stats" id="queue-stats-display">${t('queueStats', { selected: 0, total: 0 })}</span>
          </div>
          <div class="video-list-container" id="video-list">
            <div style="padding: 20px; text-align: center; color: var(--text-secondary); font-size: 13px;">
              ${t('queueEmpty')}
            </div>
          </div>
        </div>

        <div class="action-section">
          <button type="button" id="start-btn" class="btn btn-primary" disabled>${t('start')}</button>
          <button type="button" id="cancel-btn" class="btn btn-danger">${t('cancel')}</button>
        </div>

        <div class="monitor-card" id="monitor-card" style="display: none;">
          <div class="progress-header">
            <span id="active-video-title" style="font-weight: 600; max-width: 50%; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">${t('processingVideo')}</span>
            <div class="progress-meta">
              <span id="download-speed">- MB/s</span>
              <span id="download-eta">${t('etaIdle')}</span>
              <span id="download-percent" style="color: var(--primary-color); font-weight: 700;">0%</span>
            </div>
          </div>
          <div class="progress-bar-container">
            <div id="progress-bar" class="progress-bar-fill"></div>
          </div>
        </div>

        <div class="console-card">
          <div class="console-header">
            <span>${t('logTitle')}</span>
            <span id="console-status" style="font-weight: normal; color: var(--text-secondary);">${t('statusIdle')}</span>
          </div>
          <div class="console-body" id="console-log">${t('startedLog')}</div>
        </div>
      </div>
    </div>
  `;

  // Get DOM elements
  const channelUrlInput = document.getElementById('channel-url') as HTMLInputElement;
  const cookiesBrowserSelect = document.getElementById('cookies-browser') as HTMLSelectElement;
  const cookieLockWarningEl = document.getElementById('cookie-lock-warning') as HTMLDivElement;
  const downloadDirInput = document.getElementById('download-dir') as HTMLInputElement;
  const browseDirBtn = document.getElementById('browse-dir') as HTMLButtonElement;
  const fetchListBtn = document.getElementById('fetch-list-btn') as HTMLButtonElement;
  
  const optVideoCheckbox = document.getElementById('opt-video') as HTMLInputElement;
  const optAudioCheckbox = document.getElementById('opt-audio') as HTMLInputElement;
  const ffmpegWarningEl = document.getElementById('ffmpeg-warning') as HTMLDivElement;
  const audioFormatGroupEl = document.getElementById('audio-format-group') as HTMLDivElement;
  const audioFormatSelect = document.getElementById('audio-format') as HTMLSelectElement;
  const delaySecondsInput = document.getElementById('delay-seconds') as HTMLInputElement;
  
  const videoListEl = document.getElementById('video-list') as HTMLDivElement;
  const channelTitleDisplay = document.getElementById('channel-title-display') as HTMLSpanElement;
  const queueStatsDisplay = document.getElementById('queue-stats-display') as HTMLSpanElement;
  
  const startBtn = document.getElementById('start-btn') as HTMLButtonElement;
  const cancelBtn = document.getElementById('cancel-btn') as HTMLButtonElement;
  
  const monitorCardEl = document.getElementById('monitor-card') as HTMLDivElement;
  const activeVideoTitleEl = document.getElementById('active-video-title') as HTMLSpanElement;
  const downloadSpeedEl = document.getElementById('download-speed') as HTMLSpanElement;
  const downloadEtaEl = document.getElementById('download-eta') as HTMLSpanElement;
  const downloadPercentEl = document.getElementById('download-percent') as HTMLSpanElement;
  const progressBarEl = document.getElementById('progress-bar') as HTMLDivElement;
  
  const consoleLogEl = document.getElementById('console-log') as HTMLDivElement;
  const consoleStatusEl = document.getElementById('console-status') as HTMLSpanElement;
  const jsRuntimeStatusBadge = document.getElementById('js-runtime-status') as HTMLDivElement;
  const ffmpegStatusBadge = document.getElementById('ffmpeg-status') as HTMLDivElement;

  // Log utility
  function addLog(message: string, isError = false) {
    const timestamp = new Date().toLocaleTimeString();
    const prefix = isError ? '[ERROR] ' : '';
    const line = document.createElement('div');
    line.style.color = isError ? 'var(--danger-color)' : 'var(--text-primary)';
    line.style.whiteSpace = 'pre-wrap';
    line.textContent = `[${timestamp}] ${prefix}${message}`;
    consoleLogEl.appendChild(line);
    consoleLogEl.scrollTop = consoleLogEl.scrollHeight;
  }

  function isChromiumCookieBrowser(value: string) {
    return value === 'chrome' || value === 'edge';
  }

  function updateCookieLockWarning() {
    if (isChromiumCookieBrowser(cookiesBrowserSelect.value)) {
      cookieLockWarningEl.classList.add('visible');
    } else {
      cookieLockWarningEl.classList.remove('visible');
    }
  }

  cookiesBrowserSelect.addEventListener('change', updateCookieLockWarning);
  updateCookieLockWarning();

  // Update check box interactions
  function updateMediaSelections() {
    const videoChecked = optVideoCheckbox.checked;
    const audioChecked = optAudioCheckbox.checked;
    
    // Toggle warning box
    if (videoChecked || audioChecked) {
      ffmpegWarningEl.classList.add('visible');
    } else {
      ffmpegWarningEl.classList.remove('visible');
    }

    // Toggle audio format selector
    if (audioChecked) {
      audioFormatGroupEl.classList.add('visible');
    } else {
      audioFormatGroupEl.classList.remove('visible');
    }
  }

  optVideoCheckbox.addEventListener('change', updateMediaSelections);
  optAudioCheckbox.addEventListener('change', updateMediaSelections);

  // Check Environment (JS runtime and FFmpeg availability)
  try {
    const env: {
      ffmpeg_installed: boolean;
      js_runtime_installed: boolean;
      js_runtime_name: string | null;
    } = await invoke('check_environment');

    if (env.js_runtime_installed) {
      const runtimeLabel = env.js_runtime_name
        ? env.js_runtime_name.charAt(0).toUpperCase() + env.js_runtime_name.slice(1)
        : 'Runtime';
      jsRuntimeStatusBadge.textContent = t('jsRuntimeFound', { name: runtimeLabel });
      jsRuntimeStatusBadge.className = 'status-badge ok';
      addLog(t('jsRuntimeFoundLog', { name: runtimeLabel }));
    } else {
      jsRuntimeStatusBadge.textContent = t('jsRuntimeMissing');
      jsRuntimeStatusBadge.className = 'status-badge warn';
      addLog(t('jsRuntimeMissingLog'), true);
    }

    if (env.ffmpeg_installed) {
      ffmpegStatusBadge.textContent = t('ffmpegFound');
      ffmpegStatusBadge.className = 'status-badge ok';
      addLog(t('ffmpegFoundLog'));
    } else {
      ffmpegStatusBadge.textContent = t('ffmpegMissing');
      ffmpegStatusBadge.className = 'status-badge warn';
      addLog(t('ffmpegMissingLog'), true);
    }
  } catch (err) {
    addLog(t('envCheckError', { error: String(err) }), true);
  }

  // Browse Directory dialog
  browseDirBtn.addEventListener('click', async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: t('browseDirTitle'),
      });
      if (selected && typeof selected === 'string') {
        selectedDir = selected;
        downloadDirInput.value = selected;
        addLog(t('dirChanged', { path: selected }));
      }
    } catch (err) {
      addLog(t('browseDirError', { error: String(err) }), true);
    }
  });

  // Fetch playlist / channel video archive list
  fetchListBtn.addEventListener('click', async () => {
    const url = channelUrlInput.value.trim();
    if (!url) {
      addLog(t('urlRequired'), true);
      return;
    }

    fetchListBtn.disabled = true;
    fetchListBtn.textContent = t('fetchingList');
    consoleStatusEl.textContent = t('statusFetching');
    addLog(t('fetchingListLog', { url }));
    if (isChromiumCookieBrowser(cookiesBrowserSelect.value)) {
      addLog(t('cookieLockHint'));
    }
    
    videoListEl.innerHTML = `<div style="padding: 20px; text-align: center; color: var(--text-secondary);">${t('parsingVideos')}</div>`;

    try {
      const info: ChannelInfo = await invoke('get_channel_videos', {
        url,
        cookiesBrowser: cookiesBrowserSelect.value,
        locale: locale(),
      });

      fetchedVideos = info.videos;
      currentChannelTitle = info.channel_title;
      channelTitleDisplay.textContent = t('queueTitleWithChannel', { channel: info.channel_title });
      
      if (fetchedVideos.length === 0) {
        videoListEl.innerHTML = `<div style="padding: 20px; text-align: center; color: var(--text-secondary);">${t('noArchives')}</div>`;
        startBtn.disabled = true;
        addLog(t('listEmpty'), true);
      } else {
        renderQueue();
        startBtn.disabled = false;
        addLog(t('listSuccess', { count: fetchedVideos.length }));
      }
    } catch (err) {
      const errText = String(err);
      videoListEl.replaceChildren();
      const errorBox = document.createElement('div');
      errorBox.style.padding = '20px';
      errorBox.style.color = 'var(--danger-color)';
      errorBox.style.whiteSpace = 'pre-wrap';
      errorBox.style.fontSize = '13px';
      errorBox.style.lineHeight = '1.5';
      errorBox.textContent = `${t('listFailed')}\n\n${errText}`;
      videoListEl.appendChild(errorBox);
      startBtn.disabled = true;
      addLog(t('listFailedLog', { error: errText }), true);
    } finally {
      fetchListBtn.disabled = false;
      fetchListBtn.textContent = t('fetchList');
      consoleStatusEl.textContent = t('statusIdle');
    }
  });

  // Render the video rows inside the queue card
  function renderQueue() {
    videoListEl.innerHTML = '';
    
    // Add "Select All" control header
    const selectAllRow = document.createElement('div');
    selectAllRow.className = 'video-row';
    selectAllRow.style.borderBottom = '2px solid var(--border-color)';
    selectAllRow.style.background = '#f8fafc';
    selectAllRow.innerHTML = `
      <input type="checkbox" id="select-all-videos" checked />
      <span class="video-title-text" style="font-weight: 600;">${t('selectAll')}</span>
    `;
    videoListEl.appendChild(selectAllRow);

    fetchedVideos.forEach(video => {
      const row = document.createElement('div');
      row.className = 'video-row';
      row.id = `video-${video.id}`;
      row.innerHTML = `
        <input type="checkbox" class="video-select-cb" data-id="${video.id}" checked />
        <span class="video-title-text" title="${video.title}">${video.title}</span>
        <span class="video-status-text">${t('statusWaiting')}</span>
      `;
      videoListEl.appendChild(row);
    });

    const selectAllCheckbox = document.getElementById('select-all-videos') as HTMLInputElement;
    const itemCheckboxes = document.querySelectorAll('.video-select-cb') as NodeListOf<HTMLInputElement>;

    selectAllCheckbox.addEventListener('change', () => {
      const checked = selectAllCheckbox.checked;
      itemCheckboxes.forEach(cb => {
        cb.checked = checked;
      });
      updateQueueStats();
    });

    itemCheckboxes.forEach(cb => {
      cb.addEventListener('change', () => {
        const checkedCount = Array.from(itemCheckboxes).filter(c => c.checked).length;
        selectAllCheckbox.checked = checkedCount === itemCheckboxes.length;
        updateQueueStats();
      });
    });

    updateQueueStats();
  }

  function updateQueueStats() {
    const itemCheckboxes = document.querySelectorAll('.video-select-cb') as NodeListOf<HTMLInputElement>;
    const checkedCount = Array.from(itemCheckboxes).filter(c => c.checked).length;
    queueStatsDisplay.textContent = t('queueStats', { selected: checkedCount, total: fetchedVideos.length });
    
    if (checkedCount > 0 && !isDownloading) {
      startBtn.disabled = false;
    } else {
      startBtn.disabled = true;
    }
  }

  // Get currently selected videos
  function getSelectedVideos(): VideoInfo[] {
    const itemCheckboxes = document.querySelectorAll('.video-select-cb') as NodeListOf<HTMLInputElement>;
    const selectedIds = new Set(
      Array.from(itemCheckboxes)
        .filter(cb => cb.checked)
        .map(cb => cb.getAttribute('data-id') || '')
    );
    return fetchedVideos.filter(video => selectedIds.has(video.id));
  }

  // Start download loop execution
  startBtn.addEventListener('click', async () => {
    const selectedVideos = getSelectedVideos();
    if (selectedVideos.length === 0) {
      addLog(t('noVideosSelected'), true);
      return;
    }

    // Capture settings
    const options = {
      chat: (document.getElementById('opt-chat') as HTMLInputElement).checked,
      metadata: (document.getElementById('opt-metadata') as HTMLInputElement).checked,
      description: (document.getElementById('opt-description') as HTMLInputElement).checked,
      subtitles: (document.getElementById('opt-subtitles') as HTMLInputElement).checked,
      thumbnail: (document.getElementById('opt-thumbnail') as HTMLInputElement).checked,
      video: (document.getElementById('opt-video') as HTMLInputElement).checked,
      audio: (document.getElementById('opt-audio') as HTMLInputElement).checked,
      audio_format: audioFormatSelect.value,
      cookies_browser: cookiesBrowserSelect.value,
    };

    const delaySeconds = parseInt(delaySecondsInput.value) || 0;

    // Toggle downloading state variables
    isDownloading = true;
    startBtn.disabled = true;
    cancelBtn.style.display = 'block';
    fetchListBtn.disabled = true;
    cookiesBrowserSelect.disabled = true;
    channelUrlInput.disabled = true;
    
    // Disable selections
    document.querySelectorAll('.video-select-cb').forEach(el => (el as HTMLInputElement).disabled = true);
    (document.getElementById('select-all-videos') as HTMLInputElement).disabled = true;

    monitorCardEl.style.display = 'block';
    consoleStatusEl.textContent = t('statusDownloading');
    addLog(t('downloadStarted', { count: selectedVideos.length }));
    if (isChromiumCookieBrowser(cookiesBrowserSelect.value)) {
      addLog(t('cookieLockHintDownload'));
    }

    try {
      await invoke('start_download_archive', {
        options,
        videos: selectedVideos.map(video => ({ id: video.id, url: video.url, title: video.title })),
        channelTitle: currentChannelTitle,
        delaySeconds,
        customDir: selectedDir,
        locale: locale(),
      });
      addLog(t('allDone'));
      consoleStatusEl.textContent = t('statusDone');
    } catch (err) {
      if (err === 'Cancelled') {
        addLog(t('downloadCancelled'), true);
        consoleStatusEl.textContent = t('statusCancelled');
      } else {
        addLog(t('downloadError', { error: String(err) }), true);
        consoleStatusEl.textContent = t('statusError');
      }
    } finally {
      resetUiState();
    }
  });

  // Cancel download button click handler
  cancelBtn.addEventListener('click', async () => {
    cancelBtn.disabled = true;
    cancelBtn.textContent = t('cancelling');
    addLog(t('cancelRequested'));
    
    try {
      await invoke('cancel_downloads');
    } catch (err) {
      addLog(t('cancelError', { error: String(err) }), true);
    }
  });

  // Reset UI back to idle state
  function resetUiState() {
    isDownloading = false;
    startBtn.disabled = false;
    cancelBtn.style.display = 'none';
    cancelBtn.disabled = false;
    cancelBtn.textContent = t('cancel');
    fetchListBtn.disabled = false;
    cookiesBrowserSelect.disabled = false;
    channelUrlInput.disabled = false;

    // Enable checkboxes
    document.querySelectorAll('.video-select-cb').forEach(el => (el as HTMLInputElement).disabled = false);
    const selectAllCheckbox = document.getElementById('select-all-videos') as HTMLInputElement;
    if (selectAllCheckbox) selectAllCheckbox.disabled = false;

    monitorCardEl.style.display = 'none';
    progressBarEl.style.width = '0%';
    progressBarEl.className = 'progress-bar-fill';
    activeVideoTitleEl.textContent = '-';
    downloadSpeedEl.textContent = '- MB/s';
    downloadEtaEl.textContent = t('etaIdle');
    downloadPercentEl.textContent = '0%';

    updateQueueStats();
  }

  // Tauri Event Listeners
  
  // Highlight row on video start
  await listen<string>('video-started', (event) => {
    const videoId = event.payload;
    const row = document.getElementById(`video-${videoId}`);
    if (row) {
      row.className = 'video-row selected';
      const statusText = row.querySelector('.video-status-text');
      if (statusText) statusText.textContent = t('statusWorking');
    }
    
    // Find the title matching the videoId
    const video = fetchedVideos.find(v => v.id === videoId);
    if (video) {
      activeVideoTitleEl.textContent = video.title;
      addLog(t('downloadStartLog', { title: video.title }));
    }
    
    // Reset progress bar for next item
    progressBarEl.style.width = '0%';
    progressBarEl.className = 'progress-bar-fill';
    downloadPercentEl.textContent = '0%';
    downloadSpeedEl.textContent = '- MB/s';
    downloadEtaEl.textContent = t('etaIdle');
  });

  // Highlight row on video completion
  await listen<string>('video-finished', (event) => {
    const videoId = event.payload;
    const row = document.getElementById(`video-${videoId}`);
    if (row) {
      row.className = 'video-row completed';
      const statusText = row.querySelector('.video-status-text');
      if (statusText) statusText.textContent = t('statusComplete');
    }
    
    const video = fetchedVideos.find(v => v.id === videoId);
    if (video) {
      addLog(t('downloadFinishLog', { title: video.title }));
    }

    progressBarEl.className = 'progress-bar-fill finished';
    progressBarEl.style.width = '100%';
    downloadPercentEl.textContent = '100%';
  });

  // Track progress of active video item download
  await listen<ProgressPayload>('download-progress', (event) => {
    const progress = event.payload;
    progressBarEl.style.width = `${progress.percentage}%`;
    downloadPercentEl.textContent = `${Math.floor(progress.percentage)}%`;
    if (progress.speed) downloadSpeedEl.textContent = progress.speed;
    if (progress.eta) downloadEtaEl.textContent = t('etaRemaining', { eta: progress.eta });
  });

  // Append raw download stdout/stderr line logs
  await listen<ProgressPayload>('download-log', (event) => {
    const payload = event.payload;
    if (payload.log) {
      // Clean up log line (removing CR or leading spaces)
      const cleanLine = payload.log.replace(/\r/g, '').trim();
      if (cleanLine.length > 0) {
        addLog(cleanLine);
      }
    }
  });
});
