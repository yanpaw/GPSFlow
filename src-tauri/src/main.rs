// Prevents a console window from appearing alongside the app on Windows.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::{Path, PathBuf};
use std::sync::Mutex;

use serde::Serialize;

// Both traits are only needed by the macOS file-open path below, so gate the
// imports to keep other platforms warning-free.
#[cfg(any(target_os = "macos", target_os = "ios"))]
use tauri::{Emitter, Manager};

/// A track file handed to the app, already read into memory.
///
/// `name` is the file stem, used as the session label in the UI.
/// `kind` is "gpx" (contents is XML text) or "sbp" (contents is base64,
/// because .sbp is binary and can't travel through the JSON bridge as text).
#[derive(Clone, Serialize)]
struct OpenedFile {
    name: String,
    kind: String,
    contents: String,
}

/// Holds a file that arrived before the web view was ready to receive it.
/// The frontend drains this once on startup via `take_opened_file`.
struct PendingFile(Mutex<Option<OpenedFile>>);

/// Minimal base64 encoder, so .sbp bytes can cross the JSON bridge without
/// pulling in a dependency for thirty lines of work.
fn base64(bytes: &[u8]) -> String {
    const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity((bytes.len() + 2) / 3 * 4);
    for chunk in bytes.chunks(3) {
        let b = [
            chunk[0],
            *chunk.get(1).unwrap_or(&0),
            *chunk.get(2).unwrap_or(&0),
        ];
        let n = ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | b[2] as u32;
        out.push(T[(n >> 18) as usize & 63] as char);
        out.push(T[(n >> 12) as usize & 63] as char);
        out.push(if chunk.len() > 1 { T[(n >> 6) as usize & 63] as char } else { '=' });
        out.push(if chunk.len() > 2 { T[n as usize & 63] as char } else { '=' });
    }
    out
}

/// Read a track file, if it is one we understand.
///
/// The OS can pass anything through a file association or the command line,
/// so the extension is checked before touching the contents.
fn read_track(path: &Path) -> Option<OpenedFile> {
    let ext = path.extension().and_then(|e| e.to_str())?.to_ascii_lowercase();

    // The full file name, used only as the session label in the UI. Nothing
    // depends on it: both formats carry their own timestamps.
    let name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("Session")
        .to_string();

    match ext.as_str() {
        "gpx" => {
            let contents = std::fs::read_to_string(path).ok()?;
            Some(OpenedFile { name, kind: "gpx".into(), contents })
        }
        "sbp" => {
            let bytes = std::fs::read(path).ok()?;
            Some(OpenedFile { name, kind: "sbp".into(), contents: base64(&bytes) })
        }
        _ => None,
    }
}

/// Windows and Linux pass the double-clicked file as the first argument.
fn file_from_args() -> Option<OpenedFile> {
    let path: PathBuf = std::env::args_os().nth(1)?.into();
    read_track(&path)
}

/// Store a file for the frontend to collect.
///
/// Each step is its own statement on purpose: binding the `State` and the
/// `MutexGuard` to named locals keeps both alive for the whole function,
/// instead of the guard outliving a temporary and upsetting the borrow
/// checker (that was error E0597 in the first version).
#[cfg(any(target_os = "macos", target_os = "ios"))]
fn stash(app: &tauri::AppHandle, file: OpenedFile) {
    let state = app.state::<PendingFile>();
    let mut slot = match state.0.lock() {
        Ok(slot) => slot,
        Err(poisoned) => poisoned.into_inner(),
    };
    *slot = Some(file);
}

/// Called once by the frontend when it loads. Returns a file if the app was
/// launched by opening one, otherwise `None` (the normal dropzone flow).
#[tauri::command]
fn take_opened_file(pending: tauri::State<'_, PendingFile>) -> Option<OpenedFile> {
    let mut slot = match pending.0.lock() {
        Ok(slot) => slot,
        Err(poisoned) => poisoned.into_inner(),
    };
    slot.take()
}

fn main() {
    // Read argv before the builder starts, so the initial file can simply be
    // handed to `manage()`. This avoids touching app state inside `setup()`,
    // which is where the lifetime problem came from.
    let initial = PendingFile(Mutex::new(file_from_args()));

    let app = tauri::Builder::default()
        .manage(initial)
        .invoke_handler(tauri::generate_handler![take_opened_file])
        .build(tauri::generate_context!())
        .expect("error building GPSFlow");

    app.run(move |_app_handle, _event| {
        // macOS delivers file opens as an event rather than an argv entry,
        // both at launch and while the app is already running.
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        if let tauri::RunEvent::Opened { urls } = &_event {
            for url in urls {
                let path = match url.to_file_path() {
                    Ok(p) => p,
                    Err(_) => continue,
                };
                let file = match read_track(&path) {
                    Some(f) => f,
                    None => continue,
                };

                // If a window is up, hand it over directly; if not, park it
                // for the frontend to collect on startup.
                if _app_handle.webview_windows().is_empty() {
                    stash(_app_handle, file);
                } else if let Err(e) = _app_handle.emit("gpx-opened", file) {
                    eprintln!("failed to emit gpx-opened: {e}");
                }
            }
        }
    });
}
