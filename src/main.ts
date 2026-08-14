import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { open } from '@tauri-apps/plugin-dialog';
import { VideoInfo, ChannelInfo, ProgressPayload } from './types';
import { initI18n, locale, setLocale, t, applyDomI18n, Locale } from './i18n';

// State management variables
let fetchedVideos: VideoInfo[] = [];
let selectedDir: string | null = null;
let currentChannelTitle = "Channel";
let isDownloading = false;
const checkedVideoIds = new Set<string>();
const videoStatusById = new Map<string, 'waiting' | 'working' | 'complete'>();

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
      <div class="header-actions">
        <label class="language-switch" for="ui-locale">
          <span data-i18n="languageLabel">${t('languageLabel')}</span>
          <select id="ui-locale">
            <option value="ja">日本語</option>
            <option value="en">English</option>
          </select>
        </label>
        <div class="status-badges">
          <div id="js-runtime-status" class="status-badge warn">${t('jsRuntimeChecking')}</div>
          <div id="ffmpeg-status" class="status-badge warn">${t('ffmpegChecking')}</div>
        </div>
      </div>
    </header>
    <div class="main-container">
      <div class="left-pane">
        <div class="form-group">
          <label for="channel-url" data-i18n="urlLabel">${t('urlLabel')}</label>
          <input type="text" id="channel-url" placeholder="https://www.youtube.com/..." />
        </div>
        
        <div class="form-group">
          <label for="cookies-browser" data-i18n="cookieLabel">${t('cookieLabel')}</label>
          <select id="cookies-browser">
            <option value="none" data-i18n="cookieNone">${t('cookieNone')}</option>
            <option value="firefox" data-i18n="cookieFirefox">${t('cookieFirefox')}</option>
            <option value="chrome" data-i18n="cookieChrome">${t('cookieChrome')}</option>
            <option value="edge" data-i18n="cookieEdge">${t('cookieEdge')}</option>
            <option value="safari" data-i18n="cookieSafari">${t('cookieSafari')}</option>
          </select>
          <div id="cookie-lock-warning" class="warning-box" data-i18n="cookieLockWarning">
            ${t('cookieLockWarning')}
          </div>
        </div>

        <button type="button" id="fetch-list-btn" class="btn btn-secondary">${t('fetchList')}</button>

        <div class="form-group">
          <label data-i18n="saveDirLabel">${t('saveDirLabel')}</label>
          <div class="input-wrapper">
            <input type="text" id="download-dir" readonly data-i18n-placeholder="saveDirPlaceholder" placeholder="${t('saveDirPlaceholder')}" />
            <button type="button" id="browse-dir" class="browse-btn" data-i18n="browseDir">${t('browseDir')}</button>
            <button type="button" id="open-dir" class="browse-btn" data-i18n="openDir">${t('openDir')}</button>
          </div>
        </div>

        <div class="checkbox-card">
          <h3 data-i18n="dataSelect">${t('dataSelect')}</h3>
          <label class="checkbox-option">
            <input type="checkbox" id="opt-metadata" checked /> <span data-i18n="optMetadata">${t('optMetadata')}</span>
          </label>
          <label class="checkbox-option">
            <input type="checkbox" id="opt-chat" checked /> <span data-i18n="optChat">${t('optChat')}</span>
          </label>
          <label class="checkbox-option">
            <input type="checkbox" id="opt-description" checked /> <span data-i18n="optDescription">${t('optDescription')}</span>
          </label>
          <label class="checkbox-option">
            <input type="checkbox" id="opt-subtitles" checked /> <span data-i18n="optSubtitles">${t('optSubtitles')}</span>
          </label>
          <label class="checkbox-option">
            <input type="checkbox" id="opt-thumbnail" checked /> <span data-i18n="optThumbnail">${t('optThumbnail')}</span>
          </label>
          <label class="checkbox-option">
            <input type="checkbox" id="opt-video" /> <span data-i18n="optVideo">${t('optVideo')}</span>
          </label>
          <label class="checkbox-option">
            <input type="checkbox" id="opt-audio" /> <span data-i18n="optAudio">${t('optAudio')}</span>
          </label>
          <label class="checkbox-option">
            <input type="checkbox" id="opt-csv" /> <span data-i18n="optCsv">${t('optCsv')}</span>
          </label>
          <div id="ffmpeg-warning" class="warning-box" data-i18n-html="ffmpegWarning">
            ${t('ffmpegWarning')}
          </div>
          <div id="audio-format-group" class="audio-format-group">
            <label for="audio-format" data-i18n="audioFormat">${t('audioFormat')}</label>
            <select id="audio-format">
              <option value="mp3">MP3</option>
              <option value="m4a">M4A</option>
            </select>
          </div>
        </div>

        <div class="form-group">
          <label for="delay-seconds" data-i18n="delayLabel">${t('delayLabel')}</label>
          <input type="number" id="delay-seconds" min="0" max="60" value="5" style="padding: 10px 14px; border-radius: 8px; border: 1px solid var(--border-color); outline: none;" />
        </div>
      </div>
      
      <div class="right-pane">
        <div class="queue-card">
          <div class="queue-header">
            <span class="queue-title" id="channel-title-display">${t('queueTitle')}</span>
            <span class="queue-stats" id="queue-stats-display">${t('queueStats', { selected: 0, total: 0 })}</span>
          </div>
          <div class="queue-filters" id="queue-filters">
            <label class="queue-filter">
              <span data-i18n="filterTitle">${t('filterTitle')}</span>
              <input type="search" id="filter-title" data-i18n-placeholder="filterTitlePlaceholder" placeholder="${t('filterTitlePlaceholder')}" />
            </label>
            <label class="queue-filter">
              <span data-i18n="filterDateFrom">${t('filterDateFrom')}</span>
              <input type="date" id="filter-date-from" />
            </label>
            <label class="queue-filter">
              <span data-i18n="filterDateTo">${t('filterDateTo')}</span>
              <input type="date" id="filter-date-to" />
            </label>
            <label class="queue-filter">
              <span data-i18n="filterAvailability">${t('filterAvailability')}</span>
              <select id="filter-availability">
                <option value="all" data-i18n="filterAvailabilityAll">${t('filterAvailabilityAll')}</option>
                <option value="public" data-i18n="filterAvailabilityPublic">${t('filterAvailabilityPublic')}</option>
                <option value="members" data-i18n="filterAvailabilityMembers">${t('filterAvailabilityMembers')}</option>
              </select>
            </label>
          </div>
          <div class="video-list-container" id="video-list">
            <div style="padding: 20px; text-align: center; color: var(--text-secondary); font-size: 13px;">
              ${t('queueEmpty')}
            </div>
          </div>
        </div>

        <div class="action-section">
          <button type="button" id="start-btn" class="btn btn-primary" data-i18n="start" disabled>${t('start')}</button>
          <button type="button" id="cancel-btn" class="btn btn-danger">${t('cancel')}</button>
        </div>

        <div class="monitor-card" id="monitor-card" style="display: none;">
          <div class="progress-header">
            <span id="active-video-title" style="font-weight: 600; max-width: 50%; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">${t('processingVideo')}</span>
            <div class="progress-meta">
              <span id="download-count">${t('progressCount', { current: 0, total: 0 })}</span>
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
            <span data-i18n="logTitle">${t('logTitle')}</span>
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
  const openDirBtn = document.getElementById('open-dir') as HTMLButtonElement;
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
  const filterTitleInput = document.getElementById('filter-title') as HTMLInputElement;
  const filterDateFromInput = document.getElementById('filter-date-from') as HTMLInputElement;
  const filterDateToInput = document.getElementById('filter-date-to') as HTMLInputElement;
  const filterAvailabilitySelect = document.getElementById('filter-availability') as HTMLSelectElement;
  
  const startBtn = document.getElementById('start-btn') as HTMLButtonElement;
  const cancelBtn = document.getElementById('cancel-btn') as HTMLButtonElement;
  
  const monitorCardEl = document.getElementById('monitor-card') as HTMLDivElement;
  const activeVideoTitleEl = document.getElementById('active-video-title') as HTMLSpanElement;
  const downloadSpeedEl = document.getElementById('download-speed') as HTMLSpanElement;
  const downloadEtaEl = document.getElementById('download-eta') as HTMLSpanElement;
  const downloadPercentEl = document.getElementById('download-percent') as HTMLSpanElement;
  const downloadCountEl = document.getElementById('download-count') as HTMLSpanElement;
  const progressBarEl = document.getElementById('progress-bar') as HTMLDivElement;
  
  const consoleLogEl = document.getElementById('console-log') as HTMLDivElement;
  const consoleStatusEl = document.getElementById('console-status') as HTMLSpanElement;
  const jsRuntimeStatusBadge = document.getElementById('js-runtime-status') as HTMLDivElement;
  const ffmpegStatusBadge = document.getElementById('ffmpeg-status') as HTMLDivElement;
  const uiLocaleSelect = document.getElementById('ui-locale') as HTMLSelectElement;
  let downloadJobCurrent = 0;
  let downloadJobTotal = 0;
  let isFetchingList = false;
  let listError: string | null = null;
  let consolePhase: 'idle' | 'fetching' | 'downloading' | 'done' | 'cancelled' | 'error' = 'idle';
  let lastEta: string | null = null;
  let envSnapshot: {
    ffmpeg_installed: boolean;
    js_runtime_installed: boolean;
    js_runtime_name: string | null;
  } | null = null;

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

  function setConsolePhase(phase: typeof consolePhase) {
    consolePhase = phase;
    const statusKey = {
      idle: 'statusIdle',
      fetching: 'statusFetching',
      downloading: 'statusDownloading',
      done: 'statusDone',
      cancelled: 'statusCancelled',
      error: 'statusError',
    } as const;
    consoleStatusEl.textContent = t(statusKey[phase]);
  }

  function refreshEnvBadges() {
    if (!envSnapshot) {
      return;
    }
    if (envSnapshot.js_runtime_installed) {
      const runtimeLabel = envSnapshot.js_runtime_name
        ? envSnapshot.js_runtime_name.charAt(0).toUpperCase() + envSnapshot.js_runtime_name.slice(1)
        : 'Runtime';
      jsRuntimeStatusBadge.textContent = t('jsRuntimeFound', { name: runtimeLabel });
      jsRuntimeStatusBadge.className = 'status-badge ok';
    } else {
      jsRuntimeStatusBadge.textContent = t('jsRuntimeMissing');
      jsRuntimeStatusBadge.className = 'status-badge warn';
    }
    if (envSnapshot.ffmpeg_installed) {
      ffmpegStatusBadge.textContent = t('ffmpegFound');
      ffmpegStatusBadge.className = 'status-badge ok';
    } else {
      ffmpegStatusBadge.textContent = t('ffmpegMissing');
      ffmpegStatusBadge.className = 'status-badge warn';
    }
  }

  function refreshQueueChrome() {
    if (fetchedVideos.length > 0) {
      channelTitleDisplay.textContent = t('queueTitleWithChannel', { channel: currentChannelTitle });
      renderQueue();
      return;
    }
    channelTitleDisplay.textContent = t('queueTitle');
    updateQueueStats();
    if (isFetchingList) {
      videoListEl.innerHTML = `<div style="padding: 20px; text-align: center; color: var(--text-secondary);">${t('parsingVideos')}</div>`;
      return;
    }
    if (listError) {
      videoListEl.replaceChildren();
      const errorBox = document.createElement('div');
      errorBox.style.padding = '20px';
      errorBox.style.color = 'var(--danger-color)';
      errorBox.style.whiteSpace = 'pre-wrap';
      errorBox.style.fontSize = '13px';
      errorBox.style.lineHeight = '1.5';
      errorBox.textContent = `${t('listFailed')}\n\n${listError}`;
      videoListEl.appendChild(errorBox);
      return;
    }
    videoListEl.innerHTML = `<div style="padding: 20px; text-align: center; color: var(--text-secondary); font-size: 13px;">${t('queueEmpty')}</div>`;
  }

  function applyUiLanguage() {
    applyDomI18n();
    fetchListBtn.textContent = isFetchingList ? t('fetchingList') : t('fetchList');
    cancelBtn.textContent = cancelBtn.disabled && isDownloading ? t('cancelling') : t('cancel');
    setConsolePhase(consolePhase);
    refreshEnvBadges();
    refreshQueueChrome();
    downloadCountEl.textContent = t('progressCount', {
      current: downloadJobCurrent,
      total: downloadJobTotal,
    });
    downloadEtaEl.textContent = lastEta ? t('etaRemaining', { eta: lastEta }) : t('etaIdle');
  }

  uiLocaleSelect.value = locale();
  uiLocaleSelect.addEventListener('change', () => {
    const next = uiLocaleSelect.value as Locale;
    if (next !== 'ja' && next !== 'en') {
      return;
    }
    setLocale(next);
    applyUiLanguage();
  });

  [filterTitleInput, filterDateFromInput, filterDateToInput, filterAvailabilitySelect].forEach(el => {
    el.addEventListener('input', () => {
      if (fetchedVideos.length > 0) {
        renderQueue();
      }
    });
    el.addEventListener('change', () => {
      if (fetchedVideos.length > 0) {
        renderQueue();
      }
    });
  });

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
    envSnapshot = env;

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

  openDirBtn.addEventListener('click', async () => {
    try {
      await invoke('open_save_folder', { customDir: selectedDir });
    } catch (err) {
      addLog(t('openDirError', { error: String(err) }), true);
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
    isFetchingList = true;
    listError = null;
    fetchListBtn.textContent = t('fetchingList');
    consolePhase = 'fetching';
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
      listError = null;
      currentChannelTitle = info.channel_title;
      channelTitleDisplay.textContent = t('queueTitleWithChannel', { channel: info.channel_title });
      checkedVideoIds.clear();
      videoStatusById.clear();
      fetchedVideos.forEach(video => {
        checkedVideoIds.add(video.id);
        videoStatusById.set(video.id, 'waiting');
      });
      filterTitleInput.value = '';
      filterDateFromInput.value = '';
      filterDateToInput.value = '';
      filterAvailabilitySelect.value = 'all';
      
      if (fetchedVideos.length === 0) {
        videoListEl.innerHTML = `<div style="padding: 20px; text-align: center; color: var(--text-secondary);">${t('noArchives')}</div>`;
        startBtn.disabled = true;
        addLog(t('listEmpty'), true);
      } else {
        renderQueue();
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
      listError = errText;
      addLog(t('listFailedLog', { error: errText }), true);
    } finally {
      isFetchingList = false;
      fetchListBtn.disabled = false;
      fetchListBtn.textContent = t('fetchList');
      consolePhase = 'idle';
      consoleStatusEl.textContent = t('statusIdle');
    }
  });

  // Render the video rows inside the queue card
  function videoDateKey(video: VideoInfo): string | null {
    const raw = video.uploaded_at?.trim();
    if (!raw) {
      return null;
    }
    if (/^\d{4}-\d{2}-\d{2}/.test(raw)) {
      return raw.slice(0, 10);
    }
    if (/^\d+$/.test(raw)) {
      const date = new Date(Number(raw) * 1000);
      if (Number.isNaN(date.getTime())) {
        return null;
      }
      return date.toISOString().slice(0, 10);
    }
    return null;
  }

  function isMembersAvailability(value: string | null): boolean {
    const normalized = (value || '').toLowerCase();
    return (
      normalized.includes('subscriber_only') ||
      normalized.includes('premium_only') ||
      normalized.includes('member')
    );
  }

  function getFilteredVideos(): VideoInfo[] {
    const titleQuery = filterTitleInput.value.trim().toLowerCase();
    const dateFrom = filterDateFromInput.value;
    const dateTo = filterDateToInput.value;
    const availability = filterAvailabilitySelect.value;

    return fetchedVideos.filter(video => {
      if (titleQuery && !video.title.toLowerCase().includes(titleQuery)) {
        return false;
      }

      const dateKey = videoDateKey(video);
      if (dateFrom || dateTo) {
        if (!dateKey) {
          return false;
        }
        if (dateFrom && dateKey < dateFrom) {
          return false;
        }
        if (dateTo && dateKey > dateTo) {
          return false;
        }
      }

      if (availability === 'members') {
        return isMembersAvailability(video.availability);
      }
      if (availability === 'public') {
        return !isMembersAvailability(video.availability);
      }
      return true;
    });
  }

  function renderQueue() {
    const visibleVideos = getFilteredVideos();
    videoListEl.innerHTML = '';

    if (visibleVideos.length === 0) {
      const empty = document.createElement('div');
      empty.style.padding = '20px';
      empty.style.textAlign = 'center';
      empty.style.color = 'var(--text-secondary)';
      empty.style.fontSize = '13px';
      empty.textContent = fetchedVideos.length === 0 ? t('queueEmpty') : t('filterNoMatch');
      videoListEl.appendChild(empty);
      updateQueueStats();
      return;
    }

    const selectAllRow = document.createElement('div');
    selectAllRow.className = 'video-row';
    selectAllRow.style.borderBottom = '2px solid var(--border-color)';
    selectAllRow.style.background = '#f8fafc';

    const selectAllCheckbox = document.createElement('input');
    selectAllCheckbox.type = 'checkbox';
    selectAllCheckbox.id = 'select-all-videos';
    const allVisibleChecked = visibleVideos.every(video => checkedVideoIds.has(video.id));
    selectAllCheckbox.checked = allVisibleChecked;

    const selectAllLabel = document.createElement('span');
    selectAllLabel.className = 'video-title-text';
    selectAllLabel.style.fontWeight = '600';
    selectAllLabel.textContent = t('selectAll');
    selectAllRow.append(selectAllCheckbox, selectAllLabel);
    videoListEl.appendChild(selectAllRow);

    visibleVideos.forEach(video => {
      const row = document.createElement('div');
      row.className = 'video-row';
      const status = videoStatusById.get(video.id) || 'waiting';
      if (status === 'working') {
        row.classList.add('selected');
      } else if (status === 'complete') {
        row.classList.add('completed');
      }
      row.id = `video-${video.id}`;

      const checkbox = document.createElement('input');
      checkbox.type = 'checkbox';
      checkbox.className = 'video-select-cb';
      checkbox.dataset.id = video.id;
      checkbox.checked = checkedVideoIds.has(video.id);
      checkbox.disabled = isDownloading;

      const title = document.createElement('span');
      title.className = 'video-title-text';
      title.title = video.title;
      title.textContent = video.title;

      const statusText = document.createElement('span');
      statusText.className = 'video-status-text';
      statusText.textContent =
        status === 'working' ? t('statusWorking') :
        status === 'complete' ? t('statusComplete') :
        t('statusWaiting');

      row.append(checkbox, title, statusText);
      videoListEl.appendChild(row);
    });

    selectAllCheckbox.disabled = isDownloading;
    selectAllCheckbox.addEventListener('change', () => {
      const checked = selectAllCheckbox.checked;
      visibleVideos.forEach(video => {
        if (checked) {
          checkedVideoIds.add(video.id);
        } else {
          checkedVideoIds.delete(video.id);
        }
      });
      videoListEl.querySelectorAll('.video-select-cb').forEach(el => {
        (el as HTMLInputElement).checked = checked;
      });
      updateQueueStats();
    });

    videoListEl.querySelectorAll('.video-select-cb').forEach(el => {
      el.addEventListener('change', () => {
        const checkbox = el as HTMLInputElement;
        const id = checkbox.dataset.id || '';
        if (checkbox.checked) {
          checkedVideoIds.add(id);
        } else {
          checkedVideoIds.delete(id);
        }
        const visibleChecked = visibleVideos.filter(video => checkedVideoIds.has(video.id)).length;
        selectAllCheckbox.checked = visibleChecked === visibleVideos.length;
        updateQueueStats();
      });
    });

    updateQueueStats();
  }

  function updateQueueStats() {
    const visibleVideos = getFilteredVideos();
    const selectedCount = visibleVideos.filter(video => checkedVideoIds.has(video.id)).length;
    if (visibleVideos.length !== fetchedVideos.length && fetchedVideos.length > 0) {
      queueStatsDisplay.textContent = t('queueStatsFiltered', {
        selected: selectedCount,
        visible: visibleVideos.length,
        total: fetchedVideos.length,
      });
    } else {
      queueStatsDisplay.textContent = t('queueStats', {
        selected: selectedCount,
        total: fetchedVideos.length,
      });
    }

    startBtn.disabled = selectedCount === 0 || isDownloading;
  }

  function getSelectedVideos(): VideoInfo[] {
    return getFilteredVideos().filter(video => checkedVideoIds.has(video.id));
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
      csv: (document.getElementById('opt-csv') as HTMLInputElement).checked,
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
    filterTitleInput.disabled = true;
    filterDateFromInput.disabled = true;
    filterDateToInput.disabled = true;
    filterAvailabilitySelect.disabled = true;
    
    // Disable selections
    document.querySelectorAll('.video-select-cb').forEach(el => (el as HTMLInputElement).disabled = true);
    (document.getElementById('select-all-videos') as HTMLInputElement).disabled = true;

    monitorCardEl.style.display = 'block';
    consolePhase = 'downloading';
    consoleStatusEl.textContent = t('statusDownloading');
    downloadJobTotal = selectedVideos.length;
    downloadJobCurrent = 0;
    downloadCountEl.textContent = t('progressCount', { current: 0, total: downloadJobTotal });
    addLog(t('downloadStarted', { count: selectedVideos.length }));
    if (isChromiumCookieBrowser(cookiesBrowserSelect.value)) {
      addLog(t('cookieLockHintDownload'));
    }

    try {
      await invoke('start_download_archive', {
        options,
        videos: selectedVideos.map(video => ({
          id: video.id,
          url: video.url,
          title: video.title,
          duration: video.duration,
          uploaded_at: video.uploaded_at,
          availability: video.availability,
        })),
        channelTitle: currentChannelTitle,
        delaySeconds,
        customDir: selectedDir,
        locale: locale(),
      });
      addLog(t('allDone'));
      consolePhase = 'done';
      consoleStatusEl.textContent = t('statusDone');
    } catch (err) {
      if (err === 'Cancelled') {
        addLog(t('downloadCancelled'), true);
        consolePhase = 'cancelled';
        consoleStatusEl.textContent = t('statusCancelled');
      } else {
        addLog(t('downloadError', { error: String(err) }), true);
        consolePhase = 'error';
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
    filterTitleInput.disabled = false;
    filterDateFromInput.disabled = false;
    filterDateToInput.disabled = false;
    filterAvailabilitySelect.disabled = false;

    // Enable checkboxes
    document.querySelectorAll('.video-select-cb').forEach(el => (el as HTMLInputElement).disabled = false);
    const selectAllCheckbox = document.getElementById('select-all-videos') as HTMLInputElement;
    if (selectAllCheckbox) selectAllCheckbox.disabled = false;

    monitorCardEl.style.display = 'none';
    progressBarEl.style.width = '0%';
    progressBarEl.className = 'progress-bar-fill';
    activeVideoTitleEl.textContent = '-';
    downloadSpeedEl.textContent = '- MB/s';
    lastEta = null;
    downloadEtaEl.textContent = t('etaIdle');
    downloadPercentEl.textContent = '0%';
    downloadCountEl.textContent = t('progressCount', { current: 0, total: 0 });

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
    videoStatusById.set(videoId, 'working');
    downloadJobCurrent += 1;
    downloadCountEl.textContent = t('progressCount', {
      current: downloadJobCurrent,
      total: downloadJobTotal,
    });
    
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
    lastEta = null;
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
    videoStatusById.set(videoId, 'complete');
    
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
    if (progress.eta) {
      lastEta = progress.eta;
      downloadEtaEl.textContent = t('etaRemaining', { eta: progress.eta });
    }
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
