/* ═══════════════════════════════════════════════════════════
   AlgoMentor IDE — main.js
   Tauri v2 globals available via withGlobalTauri:true
════════════════════════════════════════════════════════════ */
'use strict';

const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

// ── State ────────────────────────────────────────────────────
const S = {
  workspace:    null,
  currentTask:  null,     // task name (slug)
  currentFile:  null,     // absolute path to open file
  activeTab:    'code',   // 'code' | 'task'
  sidebarOpen:  true,
  chatOpen:     true,
  outputOpen:   true,
  pinned:       false,
  isRunning:    false,
  isSending:    false,
  saveTimer:    null,
  
  watchMode:    false,
  watchTimer:   null,
};

// ── Editor ───────────────────────────────────────────────────
let editor   = null;  // Monaco IStandaloneCodeEditor
let editorModel = null;

// ── DOM refs ─────────────────────────────────────────────────
const $ = id => document.getElementById(id);

const DOM = {
  // toolbar
  sidebarToggle:  $('sidebar-toggle'),
  taskPill:       $('task-pill'),
  taskDot:        $('task-dot'),
  taskLabel:      $('task-label'),
  editorTabs:     $('editor-tabs'),
  tabCode:        $('tab-code'),
  tabTask:        $('tab-task'),
  fileIndicator:  $('file-indicator'),
  langBadge:      $('lang-badge'),
  watchBtn:       $('watch-btn'),
  runBtn:         $('run-btn'),
  stopBtn:        $('stop-btn'),
  chatToggle:     $('chat-toggle'),
  pinBtn:         $('pin-btn'),
  settingsBtn:    $('settings-btn'),
  // sidebar
  sidebar:        $('sidebar'),
  taskList:       $('task-list'),
  taskEmpty:      $('task-empty'),
  newTaskBtn:     $('new-task-btn'),
  workspaceBtn:   $('workspace-btn'),
  wsLabel:        $('ws-label'),
  // editor area
  welcome:        $('welcome'),
  openWsBtn:      $('open-ws-btn'),
  monacoWrap:     $('monaco-wrap'),
  monacoContainer:$('monaco-container'),
  taskPane:       $('task-pane'),
  taskEditor:     $('task-editor'),
  taskPreview:    $('task-preview'),
  ptabEdit:       $('ptab-edit'),
  ptabPreview:    $('ptab-preview'),
  saveDescBtn:    $('save-desc-btn'),
  
  watchBubble:    $('watch-bubble'),
  wbClose:        $('wb-close'),
  wbBody:         $('wb-body'),
  
  hresizeOutput:  $('hresize-output'),
  outputPanel:    $('output-panel'),
  outputMeta:     $('output-meta'),
  outputBody:     $('output-body'),
  clearOutput:    $('clear-output'),
  toggleOutput:   $('toggle-output'),
  // chat
  chatPanel:      $('chat-panel'),
  messages:       $('messages'),
  chips:          document.querySelectorAll('.chip'),
  input:          $('input'),
  sendBtn:        $('send-btn'),
  // modals
  settingsOverlay:    $('settings-overlay'),
  settingsClose:      $('settings-close'),
  settingsCancel:     $('settings-cancel'),
  settingsSave:       $('settings-save'),
  sProvider:          $('s-provider'),
  sModel:             $('s-model'),
  sApiKey:            $('s-api-key'),
  sProgLang:          $('s-prog-lang'),
  sLang:              $('s-lang'),
  sLevel:             $('s-level'),
  apiKeyRow:          $('api-key-row'),
  newtaskOverlay:     $('newtask-overlay'),
  newtaskClose:       $('newtask-close'),
  newtaskCancel:      $('newtask-cancel'),
  newtaskCreate:      $('newtask-create'),
  ntName:             $('nt-name'),
  ntCategory:         $('nt-category'),
  ntDifficulty:       $('nt-difficulty'),
  // resize
  vresizeLeft:    $('vresize-left'),
  vresizeRight:   $('vresize-right'),
};

// ════════════════════════════════════════════════════════════
//  Monaco initialisation
// ════════════════════════════════════════════════════════════
function initMonaco() {
  window.MonacoEnvironment = {
    getWorkerUrl: function (_workerId, _label) {
      const base = 'https://cdn.jsdelivr.net/npm/monaco-editor@0.52.2/min/';
      const worker = base + 'vs/base/worker/workerMain.js';
      return `data:text/javascript;charset=utf-8,${encodeURIComponent(
        `self.MonacoEnvironment={baseUrl:'${base}'};importScripts('${worker}');`
      )}`;
    }
  };

  require(['vs/editor/editor.main'], function () {
    // Define custom dark theme matching app palette
    monaco.editor.defineTheme('algomentor', {
      base: 'vs-dark',
      inherit: true,
      rules: [
        { token: 'comment',   foreground: '6e7681', fontStyle: 'italic' },
        { token: 'keyword',   foreground: 'd2a8ff' },
        { token: 'string',    foreground: 'a5d6ff' },
        { token: 'number',    foreground: '79c0ff' },
        { token: 'type',      foreground: 'ffa657' },
        { token: 'function',  foreground: 'd2a8ff' },
        { token: 'variable',  foreground: 'e6edf3' },
        { token: 'operator',  foreground: 'ff7b72' },
      ],
      colors: {
        'editor.background':            '#0d1117',
        'editor.foreground':            '#e6edf3',
        'editor.lineHighlightBackground':'#161b22',
        'editor.selectionBackground':   '#264f78',
        'editorCursor.foreground':      '#58a6ff',
        'editorLineNumber.foreground':  '#6e7681',
        'editorLineNumber.activeForeground': '#8b949e',
        'editorGutter.background':      '#0d1117',
        'editorIndentGuide.background': '#21262d',
        'editorBracketMatch.background':'#264f7855',
        'editorBracketMatch.border':    '#58a6ff',
        'scrollbar.shadow':             '#00000080',
        'scrollbarSlider.background':   '#30363d88',
        'scrollbarSlider.hoverBackground':'#30363dcc',
      }
    });

    editor = monaco.editor.create(DOM.monacoContainer, {
      theme: 'algomentor',
      automaticLayout: false,   // we call layout() manually via ResizeObserver
      fontSize: 13.5,
      fontFamily: "'Cascadia Code', 'Fira Code', 'Consolas', monospace",
      fontLigatures: true,
      lineHeight: 22,
      minimap: { enabled: false },
      scrollBeyondLastLine: false,
      renderLineHighlight: 'gutter',
      padding: { top: 10, bottom: 10 },
      bracketPairColorization: { enabled: true },
      guides: { bracketPairs: true, indentation: true },
      suggestOnTriggerCharacters: true,
      quickSuggestions: { other: true, comments: false, strings: true },
      snippetSuggestions: 'top',
      tabSize: 4,
      insertSpaces: true,
      wordWrap: 'off',
      smoothScrolling: true,
      cursorBlinking: 'smooth',
      cursorSmoothCaretAnimation: 'on',
      renderWhitespace: 'selection',
      folding: true,
      formatOnPaste: true,
      formatOnType: false,
    });

    // Auto-save on content change (debounced 800ms)
    editor.onDidChangeModelContent(() => {
      clearTimeout(S.saveTimer);
      S.saveTimer = setTimeout(autoSave, 800);
      
      // Watch mode (debounced 4000ms after last keystroke)
      if (S.watchMode) {
        clearTimeout(S.watchTimer);
        S.watchTimer = setTimeout(triggerWatch, 4000);
      }
    });

    // F5 = Run, Ctrl+S = Save now
    editor.addCommand(monaco.KeyCode.F5, handleRun);
    editor.addCommand(
      monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyS,
      () => { clearTimeout(S.saveTimer); autoSave(); }
    );

    // ResizeObserver so Monaco stays sharp during panel resize
    new ResizeObserver(() => editor && editor.layout()).observe(DOM.monacoContainer);

    // Load initial file if task already selected
    if (S.currentFile) loadFile(S.currentFile);
  });
}

// ════════════════════════════════════════════════════════════
//  File operations
// ════════════════════════════════════════════════════════════
async function loadFile(path) {
  if (!editor) return;
  try {
    const content = await invoke('read_file', { path });
    const lang    = await invoke('get_monaco_language', { path });

    // Dispose old model and create new one
    if (editorModel) editorModel.dispose();
    editorModel = monaco.editor.createModel(content, lang,
      monaco.Uri.file(path));
    editor.setModel(editorModel);

    // Update UI
    const name = path.split(/[\\/]/).pop();
    DOM.fileIndicator.textContent = name;
    DOM.fileIndicator.title = path;
    S.currentFile = path;

    showEditorPanel('code');
  } catch (e) {
    toast(`Cannot open file: ${e}`, 'error');
  }
}

async function autoSave() {
  if (!editor || !S.currentFile) return;
  const content = editor.getValue();
  try {
    await invoke('write_file', { path: S.currentFile, content });
  } catch (e) {
    // Silent — don't interrupt coding
    console.warn('Auto-save failed:', e);
  }
}

// ════════════════════════════════════════════════════════════
//  Panel visibility helpers
// ════════════════════════════════════════════════════════════
function showEditorPanel(tab) {
  S.activeTab = tab;
  DOM.tabCode.classList.toggle('active', tab === 'code');
  DOM.tabTask.classList.toggle('active', tab === 'task');

  // Show/hide relevant panes
  DOM.welcome.style.display    = 'none';
  DOM.monacoWrap.hidden        = tab !== 'code';
  DOM.taskPane.hidden          = tab !== 'task';
  DOM.hresizeOutput.style.display = tab === 'code' ? '' : 'none';
  DOM.outputPanel.style.display   = tab === 'code' ? '' : 'none';

  if (tab === 'code' && editor) editor.layout();
}

function showWelcome() {
  DOM.welcome.style.display    = 'flex';
  DOM.monacoWrap.hidden        = true;
  DOM.taskPane.hidden          = true;
  DOM.hresizeOutput.style.display = 'none';
  DOM.outputPanel.style.display   = 'none';
  DOM.editorTabs.classList.add('hidden');
  DOM.runBtn.classList.add('hidden');
  DOM.stopBtn.classList.add('hidden');
  DOM.fileIndicator.textContent = '';
  DOM.taskDot.classList.remove('active');
  DOM.taskLabel.textContent = 'General Chat';
  DOM.taskLabel.title = '';
}

// ════════════════════════════════════════════════════════════
//  Task list
// ════════════════════════════════════════════════════════════
async function loadTasks() {
  try {
    const tasks = await invoke('get_tasks');
    DOM.taskList.innerHTML = '';

    if (!tasks.length) {
      DOM.taskList.appendChild(DOM.taskEmpty);
      DOM.taskEmpty.style.display = '';
      return;
    }

    tasks.forEach(t => {
      const el = document.createElement('div');
      el.className = 'task-item' + (t.completed ? ' done' : '');
      el.dataset.name = t.name;
      el.innerHTML = `
        <span class="task-name">${t.title || t.name}</span>
        ${t.difficulty ? `<span class="diff-badge ${t.difficulty}">${t.difficulty[0]}</span>` : ''}
      `;
      el.addEventListener('click', () => openTask(t.name, t.title || t.name));
      DOM.taskList.appendChild(el);
    });
  } catch (e) {
    console.error('loadTasks:', e);
  }
}

async function openTask(name, title) {
  try {
    // Highlight active
    document.querySelectorAll('.task-item').forEach(el => {
      el.classList.toggle('active', el.dataset.name === name);
    });

    // Backend: open task, get solution file path
    const solPath = await invoke('open_task', { taskName: name });
    S.currentTask = name;

    // Update toolbar
    DOM.editorTabs.classList.remove('hidden');
    DOM.runBtn.classList.remove('hidden');
    DOM.stopBtn.classList.remove('hidden');
    DOM.taskDot.classList.add('active');
    DOM.taskLabel.textContent = title;
    DOM.taskLabel.title = title;

    // Load solution file
    if (solPath) {
      await loadFile(solPath);
    } else {
      // No solution file yet — show task description
      await loadTaskDescription();
      showEditorPanel('task');
    }

    // Load task description in background for the Task tab
    if (solPath) loadTaskDescription();

    // Load chat history
    await loadHistory();

  } catch (e) {
    toast(`Failed to open task: ${e}`, 'error');
  }
}

async function loadTaskDescription() {
  try {
    const md = await invoke('get_task_description');
    DOM.taskEditor.value = md;
    renderTaskPreview(md);
  } catch (e) {
    DOM.taskEditor.value = '';
  }
}

function renderTaskPreview(md) {
  // Simple but functional markdown renderer
  let html = md
    .replace(/^---[\s\S]*?---\n/, '')  // strip frontmatter
    .replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;')
    // code blocks
    .replace(/```[\w]*\n([\s\S]*?)```/g, '<pre><code>$1</code></pre>')
    // inline code
    .replace(/`([^`]+)`/g, '<code>$1</code>')
    // headers
    .replace(/^### (.+)$/gm, '<h3>$1</h3>')
    .replace(/^## (.+)$/gm,  '<h2>$1</h2>')
    .replace(/^# (.+)$/gm,   '<h1>$1</h1>')
    // bold / italic
    .replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>')
    .replace(/\*(.+?)\*/g, '<em>$1</em>')
    // lists
    .replace(/^\- (.+)$/gm, '<li>$1</li>')
    .replace(/(<li>.*<\/li>\n?)+/g, '<ul>$&</ul>')
    // ordered lists
    .replace(/^\d+\. (.+)$/gm, '<li>$1</li>')
    // blockquote
    .replace(/^> (.+)$/gm, '<blockquote>$1</blockquote>')
    // hr
    .replace(/^---$/gm, '<hr>')
    // paragraphs (lines not already in a tag)
    .replace(/^(?!<[a-z])((?!^\s*$).+)$/gm, '<p>$1</p>')
    // clean double newlines
    .replace(/\n{2,}/g, '\n');

  DOM.taskPreview.innerHTML = html;
}

// ════════════════════════════════════════════════════════════
//  Code execution
// ════════════════════════════════════════════════════════════
async function handleRun() {
  if (S.isRunning || !S.currentFile || !S.currentTask) {
    if (!S.currentFile) toast('Open a task file first', 'error');
    return;
  }

  // Save first
  clearTimeout(S.saveTimer);
  await autoSave();

  // Get task dir
  const taskDir = S.currentFile.substring(0, Math.max(
    S.currentFile.lastIndexOf('/'), S.currentFile.lastIndexOf('\\')
  ));

  // Clear output
  DOM.outputBody.innerHTML = '';
  addOutputLine('system', `▶ Running ${S.currentFile.split(/[\\/]/).pop()}…`);
  if (S.outputOpen && DOM.outputPanel.classList.contains('collapsed')) {
    toggleOutputPanel();
  }

  try {
    await invoke('run_code', { file: S.currentFile, taskDir });
  } catch (e) {
    addOutputLine('error', e);
    setRunning(false);
  }
}

async function handleStop() {
  try {
    await invoke('stop_code');
  } catch (e) { console.error(e); }
}

function setRunning(running) {
  S.isRunning = running;
  DOM.runBtn.innerHTML = running
    ? '<span class="run-spinner"></span> Running…'
    : '<svg width="13" height="13" viewBox="0 0 24 24" fill="currentColor"><polygon points="5 3 19 12 5 21 5 3"/></svg> Run';
  DOM.runBtn.classList.toggle('running', running);
  DOM.stopBtn.disabled = !running;
}

function addOutputLine(stream, text) {
  // Remove placeholder hint
  const hint = DOM.outputBody.querySelector('.output-hint');
  if (hint) hint.remove();

  const el = document.createElement('div');
  el.className = `out-line-${stream}`;
  el.textContent = text;
  DOM.outputBody.appendChild(el);
  DOM.outputBody.scrollTop = DOM.outputBody.scrollHeight;
}

function toggleOutputPanel() {
  S.outputOpen = !S.outputOpen;
  DOM.outputPanel.classList.toggle('collapsed', !S.outputOpen);
  const icon = $('toggle-output-icon');
  icon.setAttribute('points', S.outputOpen ? '18 15 12 9 6 15' : '6 9 12 15 18 9');
}

// ════════════════════════════════════════════════════════════
//  Panel resize (drag handles)
// ════════════════════════════════════════════════════════════
function makeDragH(handle, topEl, getEl, cssVar) {
  // Horizontal resize (change heights of adjacent elements)
  let start, startH;
  handle.addEventListener('mousedown', e => {
    start  = e.clientY;
    startH = getEl().offsetHeight;
    handle.classList.add('dragging');
    document.body.style.userSelect = 'none';
    document.body.style.cursor = 'row-resize';
  });
  document.addEventListener('mousemove', e => {
    if (!handle.classList.contains('dragging')) return;
    const delta = start - e.clientY;
    const newH  = Math.max(28, Math.min(600, startH + delta));
    document.documentElement.style.setProperty(cssVar, newH + 'px');
    if (editor) editor.layout();
  });
  document.addEventListener('mouseup', () => {
    if (handle.classList.contains('dragging')) {
      handle.classList.remove('dragging');
      document.body.style.userSelect = '';
      document.body.style.cursor = '';
    }
  });
}

function makeDragV(handle, sideEl, cssVar, inverted = false) {
  let start, startW;
  handle.addEventListener('mousedown', e => {
    start  = e.clientX;
    startW = sideEl.offsetWidth;
    handle.classList.add('dragging');
    document.body.style.userSelect = 'none';
    document.body.style.cursor = 'col-resize';
  });
  document.addEventListener('mousemove', e => {
    if (!handle.classList.contains('dragging')) return;
    const delta = inverted ? start - e.clientX : e.clientX - start;
    const newW  = Math.max(100, Math.min(550, startW + delta));
    document.documentElement.style.setProperty(cssVar, newW + 'px');
    if (editor) editor.layout();
  });
  document.addEventListener('mouseup', () => {
    if (handle.classList.contains('dragging')) {
      handle.classList.remove('dragging');
      document.body.style.userSelect = '';
      document.body.style.cursor = '';
    }
  });
}

// ════════════════════════════════════════════════════════════
//  Chat
// ════════════════════════════════════════════════════════════
async function loadHistory() {
  try {
    DOM.messages.innerHTML = '';
    const msgs = await invoke('get_history');
    msgs.forEach(m => appendMessage(m.role, m.content, false));
  } catch (_) { DOM.messages.innerHTML = ''; }
}

function appendMessage(role, content, scroll = true) {
  const isUser = role === 'user';
  const div = document.createElement('div');
  div.className = 'msg';
  div.innerHTML = `
    <div class="msg-header ${isUser ? 'user' : 'mentor'}">
      ${isUser ? '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="margin-right:5px; vertical-align:-2px"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path><circle cx="12" cy="7" r="4"></circle></svg>You' : '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="margin-right:5px; vertical-align:-2px"><rect x="4" y="4" width="16" height="16" rx="2" ry="2"></rect><rect x="9" y="9" width="6" height="6"></rect><line x1="9" y1="1" x2="9" y2="4"></line><line x1="15" y1="1" x2="15" y2="4"></line><line x1="9" y1="20" x2="9" y2="23"></line><line x1="15" y1="20" x2="15" y2="23"></line><line x1="20" y1="9" x2="23" y2="9"></line><line x1="20" y1="14" x2="23" y2="14"></line><line x1="1" y1="9" x2="4" y2="9"></line><line x1="1" y1="14" x2="4" y2="14"></line></svg>AlgoMentor'}
    </div>
    <div class="msg-body">${isUser
      ? escapeHtml(content)
      : simpleMarkdown(content)
    }</div>
  `;
  DOM.messages.appendChild(div);
  if (scroll) DOM.messages.scrollTop = DOM.messages.scrollHeight;
}

function appendTyping() {
  const div = document.createElement('div');
  div.className = 'msg';
  div.id = 'typing-msg';
  div.innerHTML = `
    <div class="msg-header mentor"><svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="margin-right:5px; vertical-align:-2px"><rect x="4" y="4" width="16" height="16" rx="2" ry="2"></rect><rect x="9" y="9" width="6" height="6"></rect><line x1="9" y1="1" x2="9" y2="4"></line><line x1="15" y1="1" x2="15" y2="4"></line><line x1="9" y1="20" x2="9" y2="23"></line><line x1="15" y1="20" x2="15" y2="23"></line><line x1="20" y1="9" x2="23" y2="9"></line><line x1="20" y1="14" x2="23" y2="14"></line><line x1="1" y1="9" x2="4" y2="9"></line><line x1="1" y1="14" x2="4" y2="14"></line></svg>AlgoMentor</div>
    <div class="msg-body" id="typing-body">
      <div class="typing-dots"><span></span><span></span><span></span></div>
    </div>
  `;
  DOM.messages.appendChild(div);
  DOM.messages.scrollTop = DOM.messages.scrollHeight;
}

let streamBuffer = '';

function startStream() {
  appendTyping();
  streamBuffer = '';
  setChips(false);
}

function chunkStream(text) {
  streamBuffer += text;
  const body = $('typing-body');
  if (body) {
    body.innerHTML = simpleMarkdown(streamBuffer);
    DOM.messages.scrollTop = DOM.messages.scrollHeight;
  }
}

function endStream(fullText) {
  const msg = $('typing-msg');
  if (msg) {
    msg.id = '';
    const body = msg.querySelector('.msg-body');
    if (body) body.innerHTML = simpleMarkdown(fullText || streamBuffer);
  }
  setChips(true);
  streamBuffer = '';
  S.isSending = false;
  DOM.sendBtn.disabled = false;
  DOM.input.disabled = false;
}

function setChips(enabled) {
  DOM.chips.forEach(c => c.disabled = !enabled);
}

async function sendMessage(text) {
  if (!text.trim() || S.isSending) return;
  S.isSending = true;
  DOM.sendBtn.disabled = true;
  DOM.input.disabled = true;
  DOM.input.value = '';
  adjustInputHeight();

  appendMessage('user', text);
  startStream();
  try {
    await invoke('send_chat', { msg: text });
  } catch (e) {
    endStream(`<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="margin-right:4px; vertical-align:-2px"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg> ${e}`);
  }
}

async function runChip(cmd) {
  if (S.isSending) return;
  S.isSending = true;
  DOM.sendBtn.disabled = true;
  setChips(false);

  appendMessage('user', `/${cmd}`);
  startStream();

  try {
    const cmdMap = {
      hint:     'run_hint',
      explain:  'run_explain',
      bigo:     'run_complexity',
      approach: 'run_solution',
      done:     'mark_done',
    };
    await invoke(cmdMap[cmd]);
    if (cmd === 'done') {
      endStream('');
      toast('Task marked as completed!', 'success');
      loadTasks();
    }
  } catch (e) {
    endStream(`<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="margin-right:4px; vertical-align:-2px"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg> ${e}`);
  }
}

function adjustInputHeight() {
  const el = DOM.input;
  el.style.height = 'auto';
  el.style.height = Math.min(el.scrollHeight, 100) + 'px';
}

// ════════════════════════════════════════════════════════════
//  Workspace
// ════════════════════════════════════════════════════════════
async function openWorkspace() {
  try {
    const path = await invoke('pick_directory');
    if (!path) return;
    await invoke('set_workspace', { path });
    S.workspace = path;
    DOM.wsLabel.textContent = path.split(/[\\/]/).pop();
    DOM.wsLabel.title = path;
    await loadTasks();
    showWelcome();
    toast('Workspace opened', 'success');
  } catch (e) {
    toast(`Error: ${e}`, 'error');
  }
}

// ════════════════════════════════════════════════════════════
//  Settings modal
// ════════════════════════════════════════════════════════════
async function openSettings() {
  try {
    const c = await invoke('get_config');
    DOM.sProvider.value  = c.provider;
    DOM.sModel.value     = c.model;
    DOM.sProgLang.value  = c.programming_language;
    DOM.sLang.value      = c.language;
    DOM.sLevel.value     = c.level;
    DOM.sApiKey.value    = '';
    DOM.apiKeyRow.style.display = ['ollama','lmstudio'].includes(c.provider) ? 'none' : '';
    
    // update badge
    const sel = DOM.sProgLang.options[DOM.sProgLang.selectedIndex];
    if (sel) DOM.langBadge.textContent = sel.text.split(' ')[0];
    
  } catch (_) {}
  DOM.settingsOverlay.classList.remove('hidden');
}

async function saveSettings() {
  try {
    await invoke('save_config', {
      provider:           DOM.sProvider.value,
      model:              DOM.sModel.value,
      level:              DOM.sLevel.value,
      language:           DOM.sLang.value,
      programmingLanguage:DOM.sProgLang.value,
      apiKey:             DOM.sApiKey.value || null,
    });
    DOM.settingsOverlay.classList.add('hidden');
    
    // update badge
    const sel = DOM.sProgLang.options[DOM.sProgLang.selectedIndex];
    if (sel) DOM.langBadge.textContent = sel.text.split(' ')[0];
    
    toast('Settings saved', 'success');
  } catch (e) {
    toast(`Error: ${e}`, 'error');
  }
}

// ════════════════════════════════════════════════════════════
//  New Task modal
// ════════════════════════════════════════════════════════════
async function createTask() {
  const inputVal = DOM.ntName.value.trim();
  if (!inputVal) { DOM.ntName.focus(); return; }
  
  const isUrl = inputVal.includes('leetcode.com/problems/');
  let cleanUrl = inputVal;
  if (isUrl) {
    try { cleanUrl = new URL(inputVal).pathname; } catch (_) {}
  }
  const name = isUrl ? cleanUrl.replace(/\/$/, '').split('/').pop() : inputVal.toLowerCase().replace(/\s+/g,'-');
  
  try {
    let solPath;
    if (isUrl) {
      DOM.newtaskCreate.textContent = 'Importing...';
      DOM.newtaskCreate.disabled = true;
      solPath = await invoke('import_leetcode', {
        url: inputVal,
        category: DOM.ntCategory.value.trim() || null,
      });
      DOM.newtaskCreate.textContent = 'Create Task';
      DOM.newtaskCreate.disabled = false;
    } else {
      solPath = await invoke('add_task', {
        name,
        category:   DOM.ntCategory.value.trim() || null,
        difficulty: DOM.ntDifficulty.value,
      });
    }
    
    DOM.newtaskOverlay.classList.add('hidden');
    DOM.ntName.value = DOM.ntCategory.value = '';
    await loadTasks();
    // open the new task
    await openTask(name, name);
    toast(isUrl ? 'Task imported from LeetCode!' : 'Task created!', 'success');
  } catch (e) {
    DOM.newtaskCreate.textContent = 'Create Task';
    DOM.newtaskCreate.disabled = false;
    toast(`Error: ${e}`, 'error');
  }
}

// ════════════════════════════════════════════════════════════
//  Utilities
// ════════════════════════════════════════════════════════════
function escapeHtml(t) {
  return t.replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;');
}

function simpleMarkdown(md) {
  if (!md) return '';
  return md
    .replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;')
    .replace(/```[\w]*\n?([\s\S]*?)```/g, '<pre><code>$1</code></pre>')
    .replace(/`([^`]+)`/g, '<code>$1</code>')
    .replace(/^### (.+)$/gm, '<h3>$1</h3>')
    .replace(/^## (.+)$/gm, '<h2>$1</h2>')
    .replace(/^# (.+)$/gm, '<h1>$1</h1>')
    .replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>')
    .replace(/\*(.+?)\*/g, '<em>$1</em>')
    .replace(/^---$/gm, '<hr>')
    .replace(/^> (.+)$/gm, '<blockquote>$1</blockquote>')
    .replace(/^\- (.+)$/gm, '<li>$1</li>')
    .replace(/(<li>.*<\/li>)/g, '<ul>$1</ul>')
    .replace(/\n{2,}/g, '</p><p>')
    .replace(/^(?!<[a-z])((?!^\s*$).+)$/gm, '<p>$1</p>');
}

function toast(msg, type = 'info') {
  const el = document.createElement('div');
  el.className = `toast ${type}`;
  el.textContent = msg;
  document.body.appendChild(el);
  setTimeout(() => el.remove(), 3000);
}

// ════════════════════════════════════════════════════════════
//  Event wiring
// ════════════════════════════════════════════════════════════
function wireEvents() {
  // Toolbar
  DOM.sidebarToggle.addEventListener('click', () => {
    S.sidebarOpen = !S.sidebarOpen;
    DOM.sidebar.classList.toggle('collapsed', !S.sidebarOpen);
    if (editor) setTimeout(() => editor.layout(), 200);
  });
  DOM.chatToggle.addEventListener('click', () => {
    S.chatOpen = !S.chatOpen;
    DOM.chatPanel.classList.toggle('collapsed', !S.chatOpen);
    DOM.vresizeRight.style.display = S.chatOpen ? '' : 'none';
    DOM.chatToggle.classList.toggle('active', !S.chatOpen);
    if (editor) setTimeout(() => editor.layout(), 200);
  });
  DOM.pinBtn.addEventListener('click', async () => {
    S.pinned = !S.pinned;
    DOM.pinBtn.classList.toggle('active', S.pinned);
    try { await invoke('set_always_on_top', { value: S.pinned }); } catch (_) {}
  });
  DOM.settingsBtn.addEventListener('click', openSettings);
  DOM.openWsBtn.addEventListener('click', openWorkspace);
  DOM.workspaceBtn.addEventListener('click', openWorkspace);

  // Editor tabs
  DOM.tabCode.addEventListener('click', () => {
    if (S.currentFile) showEditorPanel('code');
  });
  DOM.tabTask.addEventListener('click', async () => {
    await loadTaskDescription();
    showEditorPanel('task');
  });

  // Task description editor
  DOM.ptabEdit.addEventListener('click', () => {
    DOM.ptabEdit.classList.add('active');
    DOM.ptabPreview.classList.remove('active');
    DOM.taskEditor.classList.remove('hidden');
    DOM.taskPreview.classList.add('hidden');
  });
  DOM.ptabPreview.addEventListener('click', () => {
    DOM.ptabPreview.classList.add('active');
    DOM.ptabEdit.classList.remove('active');
    DOM.taskEditor.classList.add('hidden');
    DOM.taskPreview.classList.remove('hidden');
    renderTaskPreview(DOM.taskEditor.value);
  });
  DOM.taskEditor.addEventListener('input', () => {
    clearTimeout(S.saveTimer);
    S.saveTimer = setTimeout(() => {
      invoke('save_task_description', { content: DOM.taskEditor.value }).catch(() => {});
    }, 800);
  });
  DOM.saveDescBtn.addEventListener('click', async () => {
    try {
      await invoke('save_task_description', { content: DOM.taskEditor.value });
      toast('Task description saved', 'success');
    } catch (e) { toast(`Error: ${e}`, 'error'); }
  });

  // Watch Mode
  DOM.watchBtn.addEventListener('click', () => {
    S.watchMode = !S.watchMode;
    DOM.watchBtn.dataset.active = S.watchMode;
    DOM.watchBtn.title = `Watch Mode (Auto-Mentor) - ${S.watchMode ? 'ON' : 'OFF'}`;
    if (!S.watchMode) {
      clearTimeout(S.watchTimer);
      DOM.watchBubble.classList.add('hidden');
    } else {
      toast('Watch Mode enabled: mentor will comment when you pause typing', 'success');
    }
  });
  DOM.wbClose.addEventListener('click', () => {
    DOM.watchBubble.classList.add('hidden');
  });
  DOM.langBadge.addEventListener('click', openSettings);

  // Run / Stop
  DOM.runBtn.addEventListener('click', handleRun);
  DOM.stopBtn.addEventListener('click', handleStop);
  document.addEventListener('keydown', e => {
    if (e.key === 'F5' && !S.isRunning) { e.preventDefault(); handleRun(); }
  });

  // Output panel
  DOM.clearOutput.addEventListener('click', () => {
    DOM.outputBody.innerHTML = '<span class="output-hint">▶ Run your code to see output here</span>';
    DOM.outputMeta.textContent = '';
    DOM.outputMeta.className = 'output-meta';
  });
  DOM.toggleOutput.addEventListener('click', toggleOutputPanel);

  // Chat
  DOM.chips.forEach(chip => {
    chip.addEventListener('click', () => runChip(chip.dataset.cmd));
  });
  DOM.sendBtn.addEventListener('click', () => sendMessage(DOM.input.value));
  DOM.input.addEventListener('keydown', e => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      sendMessage(DOM.input.value);
    }
  });
  DOM.input.addEventListener('input', adjustInputHeight);

  // Sidebar new task
  DOM.newTaskBtn.addEventListener('click', () => {
    if (!S.workspace) { toast('Open a workspace first', 'error'); return; }
    DOM.ntName.value = DOM.ntCategory.value = '';
    DOM.newtaskOverlay.classList.remove('hidden');
    setTimeout(() => DOM.ntName.focus(), 50);
  });

  // Settings modal
  DOM.settingsClose.addEventListener('click',  () => DOM.settingsOverlay.classList.add('hidden'));
  DOM.settingsCancel.addEventListener('click', () => DOM.settingsOverlay.classList.add('hidden'));
  DOM.settingsSave.addEventListener('click', saveSettings);
  DOM.sProvider.addEventListener('change', () => {
    DOM.apiKeyRow.style.display = ['ollama','lmstudio'].includes(DOM.sProvider.value) ? 'none' : '';
  });

  // New task modal
  DOM.newtaskClose.addEventListener('click',  () => DOM.newtaskOverlay.classList.add('hidden'));
  DOM.newtaskCancel.addEventListener('click', () => DOM.newtaskOverlay.classList.add('hidden'));
  DOM.newtaskCreate.addEventListener('click', createTask);
  DOM.ntName.addEventListener('keydown', e => { if (e.key === 'Enter') createTask(); });

  // Overlay click-outside close
  [DOM.settingsOverlay, DOM.newtaskOverlay].forEach(ov => {
    ov.addEventListener('click', e => { if (e.target === ov) ov.classList.add('hidden'); });
  });

  // Resize handles
  makeDragH(DOM.hresizeOutput, DOM.monacoWrap, () => DOM.outputPanel, '--output-h');
  makeDragV(DOM.vresizeLeft,   DOM.sidebar,    '--sidebar-w');
  makeDragV(DOM.vresizeRight,  DOM.chatPanel,  '--chat-w', true);
}

// ════════════════════════════════════════════════════════════
//  Tauri event listeners (code execution + LLM streaming)
// ════════════════════════════════════════════════════════════
async function wireTauriEvents() {
  // LLM streaming
  await listen('mentor-start', () => startStream());
  await listen('mentor-chunk', e => chunkStream(e.payload));
  await listen('mentor-done',  e => endStream(e.payload));
  await listen('mentor-error', e => { endStream(`<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="margin-right:4px; vertical-align:-2px"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg> ${e.payload}`); });

  // Watch mode streaming
  let watchBuf = '';
  await listen('watch-start', () => {
    watchBuf = '';
    DOM.watchBubble.classList.remove('hidden');
    DOM.wbBody.innerHTML = '<div class="typing-dots"><span></span><span></span><span></span></div>';
  });
  await listen('watch-chunk', e => {
    watchBuf += e.payload;
    DOM.wbBody.innerHTML = simpleMarkdown(watchBuf);
  });
  await listen('watch-done', e => {
    DOM.wbBody.innerHTML = simpleMarkdown(e.payload || watchBuf);
  });
  await listen('watch-error', e => {
    DOM.wbBody.innerHTML = `<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="margin-right:4px; vertical-align:-2px"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg> ${escapeHtml(e.payload)}`;
  });

  // Code execution
  await listen('code-start', () => {
    setRunning(true);
    DOM.outputMeta.textContent = '';
    DOM.outputMeta.className = 'output-meta';
  });

  await listen('code-out', e => {
    const { stream, line } = e.payload;
    addOutputLine(stream === 'stderr' ? 'stderr' : 'stdout', line);
  });

  await listen('code-done', e => {
    const { exitCode, durationMs, success } = e.payload;
    const ms  = durationMs < 1000 ? `${durationMs}ms` : `${(durationMs/1000).toFixed(1)}s`;
    const checkIcon = `<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="margin-right:4px; vertical-align:-1px"><polyline points="20 6 9 17 4 12"></polyline></svg>`;
    const crossIcon = `<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="margin-right:4px; vertical-align:-1px"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>`;
    const msg = success
      ? `${checkIcon} Exit 0  (${ms})`
      : `${crossIcon} Exit ${exitCode ?? '?'}  (${ms})`;
    DOM.outputMeta.innerHTML = msg;
    DOM.outputMeta.className = `output-meta ${success ? 'success' : 'error'}`;
    addOutputLine('system', `Exit code ${exitCode ?? '?'} (${ms})`);
    setRunning(false);
  });

  await listen('code-error', e => {
    addOutputLine('error', e.payload);
    setRunning(false);
  });
}

// ════════════════════════════════════════════════════════════
//  Watch Action
// ════════════════════════════════════════════════════════════
async function triggerWatch() {
  if (!editor || !S.currentFile || !S.watchMode) return;
  const code = editor.getValue().trim();
  if (!code) return;
  
  try {
    await invoke('run_watch', { code });
  } catch (e) {
    console.warn('Watch error:', e);
  }
}

// ════════════════════════════════════════════════════════════
//  Bootstrap
// ════════════════════════════════════════════════════════════
async function init() {
  wireEvents();
  await wireTauriEvents();

  // Check for last workspace
  try {
    const last = await invoke('get_last_workspace');
    if (last) {
      await invoke('set_workspace', { path: last }); // sets backend Tokio state properly
      S.workspace = last;
      DOM.wsLabel.textContent = last.split(/[\\/]/).pop();
      DOM.wsLabel.title = last;
      await loadTasks();
    }
  } catch (_) {}

  // Init Monaco (after events, no await)
  initMonaco();
}

init().catch(console.error);
