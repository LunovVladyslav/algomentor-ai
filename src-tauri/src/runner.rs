// src-tauri/src/runner.rs
// Language runner: spawns compilers/interpreters and streams output.

use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tauri::{AppHandle, Emitter};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

// ── Public type ──────────────────────────────────────────────
pub type PidStore = Arc<Mutex<Option<u32>>>;

// ── Map file extension → Monaco language ID ─────────────────
pub fn monaco_language(ext: &str) -> &'static str {
    match ext {
        "py"               => "python",
        "js" | "mjs"       => "javascript",
        "ts" | "mts"       => "typescript",
        "rs"               => "rust",
        "cpp" | "cc" | "cxx" | "c++" => "cpp",
        "c"                => "c",
        "java"             => "java",
        "go"               => "go",
        "md"               => "markdown",
        "json"             => "json",
        "toml"             => "ini",
        _                  => "plaintext",
    }
}

// ── Build (program, args) for running a file ─────────────────
fn build_run_cmd(ext: &str, file: &Path, task_dir: &Path) -> Option<(String, Vec<String>)> {
    let f    = file.to_string_lossy().into_owned();
    let stem = file.file_stem()?.to_string_lossy().into_owned();
    let dir  = task_dir.to_string_lossy().into_owned();
    let out  = task_dir.join(format!("{stem}.exe")).to_string_lossy().into_owned();

    Some(match ext {
        "py"  => ("python".into(),  vec![f]),
        "js"  => ("node".into(),    vec![f]),
        "ts"  => ("npx".into(),     vec!["--yes".into(), "ts-node".into(), f]),
        "go"  => ("go".into(),      vec!["run".into(), f]),
        "rs"  => {
            if task_dir.join("Cargo.toml").exists() {
                ("cargo".into(), vec!["run".into()])
            } else {
                ("cmd".into(), vec!["/C".into(),
                    format!("rustc --edition 2021 \"{f}\" -o \"{out}\" 2>&1 && \"{out}\"")])
            }
        }
        "cpp" | "cc" | "cxx" => ("cmd".into(), vec!["/C".into(),
            format!("g++ -std=c++17 \"{f}\" -o \"{out}\" 2>&1 && \"{out}\"")]),
        "java" => ("cmd".into(), vec!["/C".into(),
            format!("javac \"{f}\" 2>&1 && java -cp \"{dir}\" {stem}")]),
        _ => return None,
    })
}

// ── Build check-only command (syntax, no run) ─────────────────
pub fn build_check_cmd(ext: &str, file: &Path) -> Option<(String, Vec<String>)> {
    let f = file.to_string_lossy().into_owned();
    Some(match ext {
        "py"  => ("python".into(), vec!["-m".into(), "py_compile".into(), f]),
        "js"  => ("node".into(),   vec!["--check".into(), f]),
        "rs"  => ("rustc".into(),  vec!["--edition".into(), "2021".into(), "--error-format".into(), "json".into(), f]),
        "cpp" | "cc" | "cxx" => ("g++".into(), vec!["-std=c++17".into(), "-fsyntax-only".into(), f]),
        "java"=> ("javac".into(),  vec![f]),
        "go"  => ("go".into(),     vec!["vet".into(), f]),
        _ => return None,
    })
}

// ── Main: spawn process and stream output ────────────────────
pub async fn run_file(
    app:       AppHandle,
    pid_store: PidStore,
    file:      PathBuf,
    task_dir:  PathBuf,
) -> Result<(), String> {
    let ext = file.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let (prog, args) = build_run_cmd(&ext, &file, &task_dir)
        .ok_or_else(|| format!(
            "No runner configured for .{ext} files.\n\
             Supported: .py .js .ts .go .rs .cpp .java"
        ))?;

    let start = Instant::now();

    let mut child = Command::new(&prog)
        .args(&args)
        .current_dir(&task_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .map_err(|e| format!(
            "Failed to launch '{prog}': {e}\n\
             Make sure it is installed and available in PATH."
        ))?;

    if let Some(pid) = child.id() {
        *pid_store.lock().unwrap() = Some(pid);
    }
    app.emit("code-start", ()).ok();

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let app_o = app.clone();
    let app_e = app.clone();

    let h_out = tokio::spawn(async move {
        let mut lines = BufReader::new(stdout).lines();
        while let Ok(Some(line)) = lines.next_line().await {
            app_o.emit("code-out", serde_json::json!({ "stream": "stdout", "line": line })).ok();
        }
    });

    let h_err = tokio::spawn(async move {
        let mut lines = BufReader::new(stderr).lines();
        while let Ok(Some(line)) = lines.next_line().await {
            app_e.emit("code-out", serde_json::json!({ "stream": "stderr", "line": line })).ok();
        }
    });

    let status = child.wait().await.map_err(|e| e.to_string())?;
    let _ = tokio::join!(h_out, h_err);

    *pid_store.lock().unwrap() = None;

    let ms = start.elapsed().as_millis() as u64;
    app.emit("code-done", serde_json::json!({
        "exitCode": status.code(),
        "durationMs": ms,
        "success": status.success(),
    })).ok();

    Ok(())
}

// ── Kill running process by PID (Windows taskkill) ───────────
pub async fn kill_process(pid_store: PidStore) {
    let pid = pid_store.lock().unwrap().take();
    if let Some(pid) = pid {
        let _ = Command::new("taskkill")
            .args(["/F", "/PID", &pid.to_string()])
            .output()
            .await;
    }
}
