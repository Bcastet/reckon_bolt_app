//! In-app log journal: ring buffer + file append + live UI events.

use std::collections::VecDeque;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager};

const MAX_ENTRIES: usize = 2_000;
const LOG_FILE: &str = "reckon-bolt.log";

static APP: OnceLock<AppHandle> = OnceLock::new();
static ENTRIES: OnceLock<Mutex<VecDeque<JournalEntry>>> = OnceLock::new();
static LOG_PATH: OnceLock<PathBuf> = OnceLock::new();

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JournalEntry {
    pub id: u64,
    pub timestamp: String,
    pub level: String,
    pub source: String,
    pub message: String,
}

fn entries() -> &'static Mutex<VecDeque<JournalEntry>> {
    ENTRIES.get_or_init(|| Mutex::new(VecDeque::with_capacity(256)))
}

fn next_id(buf: &VecDeque<JournalEntry>) -> u64 {
    buf.back().map(|e| e.id + 1).unwrap_or(1)
}

fn now_stamp() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string()
}

/// Initialize the journal (call once during app setup).
pub fn init(app: &AppHandle) -> Result<(), String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Cannot resolve app data dir: {}", e))?;
    fs::create_dir_all(&dir).map_err(|e| format!("Cannot create app data dir: {}", e))?;

    let path = dir.join(LOG_FILE);
    let _ = LOG_PATH.set(path);
    let _ = APP.set(app.clone());

    write(
        "info",
        "App",
        &format!("Reckon Bolt started (v{})", app.config().version.clone().unwrap_or_default()),
    );
    Ok(())
}

/// Append a journal entry (also mirrors to stderr + log file + UI event).
pub fn write(level: &str, source: &str, message: &str) {
    let timestamp = now_stamp();
    let level = level.to_string();
    let source = source.to_string();
    let message = message.to_string();

    let line = format!("[{}] [{}] [{}] {}\n", timestamp, level.to_uppercase(), source, message);
    eprint!("{}", line);

    if let Some(path) = LOG_PATH.get() {
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
            let _ = file.write_all(line.as_bytes());
        }
    }

    let entry = {
        let mut buf = match entries().lock() {
            Ok(g) => g,
            Err(_) => return,
        };
        let entry = JournalEntry {
            id: next_id(&buf),
            timestamp,
            level,
            source,
            message,
        };
        buf.push_back(entry.clone());
        while buf.len() > MAX_ENTRIES {
            buf.pop_front();
        }
        entry
    };

    if let Some(app) = APP.get() {
        let _ = app.emit("journal-entry", &entry);
    }
}

pub fn info(source: &str, message: &str) {
    write("info", source, message);
}

pub fn warn(source: &str, message: &str) {
    write("warn", source, message);
}

pub fn error(source: &str, message: &str) {
    write("error", source, message);
}

#[tauri::command]
pub fn get_journal_entries() -> Vec<JournalEntry> {
    entries()
        .lock()
        .map(|buf| buf.iter().cloned().collect())
        .unwrap_or_default()
}

#[tauri::command]
pub fn clear_journal() -> Result<(), String> {
    if let Ok(mut buf) = entries().lock() {
        buf.clear();
    }
    if let Some(path) = LOG_PATH.get() {
        let _ = fs::write(path, "");
    }
    info("Journal", "Journal cleared");
    Ok(())
}

#[tauri::command]
pub fn open_journal_folder(app: AppHandle) -> Result<(), String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Cannot resolve app data dir: {}", e))?;
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(dir.to_string_lossy().as_ref())
            .spawn()
            .map_err(|e| format!("Failed to open explorer: {}", e))?;
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = dir;
        return Err("Open folder is only supported on Windows".into());
    }
    Ok(())
}

/// Frontend → journal (user actions, toasts, etc.).
#[tauri::command]
pub fn append_journal_entry(level: String, source: String, message: String) {
    write(&level, &source, &message);
}
