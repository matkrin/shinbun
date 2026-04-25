// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    io,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::{anyhow, Result};
use clap::Parser;
use notify::{
    event::{DataChange, ModifyKind},
    Watcher,
};
use tauri::Emitter;

struct MdState {
    md_file: Option<PathBuf>,
    stdin: Option<String>,
    sync: bool,
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Markdown file to open
    file: Option<String>,
    /// Listen to stdin, must be exclusive
    #[arg(short, long, exclusive = true)]
    sync: bool,
}

fn main() {
    // let stdin = read_stdin();
    let stdin = None;
    let args = Args::parse();
    let md_file = match &args.file {
        Some(file) => {
            let path_buf = PathBuf::from(file);
            if path_buf.exists() {
                Some(path_buf)
            } else {
                None
            }
        }
        None => None,
    };

    let md_state = MdState {
        md_file,
        stdin,
        sync: args.sync,
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .manage(Arc::new(Mutex::new(md_state)))
        .invoke_handler(tauri::generate_handler![
            load_markdown,
            watch_file,
            start_sync,
            exit,
            is_stdin,
            is_sync,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn is_sync(state: tauri::State<Arc<Mutex<MdState>>>) -> bool {
    let state = state.lock().expect("state mutex poisoned");
    state.sync
}

#[tauri::command]
fn is_stdin(state: tauri::State<Arc<Mutex<MdState>>>) -> bool {
    let state = state.lock().expect("state mutex poisoned");
    state.stdin.is_some()
}

#[tauri::command]
fn load_markdown(state: tauri::State<Arc<Mutex<MdState>>>) -> Result<String, String> {
    let state = state.lock().map_err(|_| "state poisoned")?;

    if let Some(md) = &state.stdin {
        return read_markdown(md).map_err(|e| e.to_string());
    }

    let path = state.md_file.as_ref().ok_or("no file")?;
    read_markdown_from_file(path).map_err(|e| e.to_string())
}

#[tauri::command]
fn watch_file(window: tauri::Window, state: tauri::State<Arc<Mutex<MdState>>>) {
    let state = Arc::clone(&state);
    let win = window.clone();

    std::thread::spawn(move || {
        let md_file = match state.lock().ok().and_then(|s| s.md_file.clone()) {
            Some(f) => f,
            None => return,
        };

        let (tx, rx) = std::sync::mpsc::channel();

        let mut watcher = match notify::recommended_watcher(tx) {
            Ok(w) => w,
            Err(_) => return,
        };

        if watcher
            .watch(&md_file, notify::RecursiveMode::NonRecursive)
            .is_err()
        {
            return;
        }
        loop {
            match rx.recv() {
                Ok(Ok(event)) => {
                    if let notify::EventKind::Modify(ModifyKind::Data(DataChange::Content)) =
                        event.kind
                    {
                        if let Ok(md) = read_markdown_from_file(&md_file) {
                            let _ = win.emit("watch", md);
                        }
                    }
                }
                _ => std::thread::sleep(Duration::from_millis(100)),
            }
        }
    });
}

#[tauri::command]
fn start_sync(window: tauri::Window) {
    let win = window.clone();
    std::thread::spawn(move || loop {
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(n) => println!("REVEIVED: {}, {}", &buffer, n),
            _ => eprintln!("Error reading from stdin"),
        }
        _ = win.emit(
            "sync",
            read_markdown(&buffer).unwrap_or("Error reading markdown".to_string()),
        );
        std::thread::sleep(Duration::from_millis(100));
    });
}

#[tauri::command]
fn exit(app_handle: tauri::AppHandle) {
    app_handle.exit(1);
}

fn read_markdown_from_file(md_file: &Path) -> Result<String> {
    let md = std::fs::read_to_string(md_file)?;
    read_markdown(&md)
}

fn read_markdown(md: &str) -> Result<String> {
    let options = markdown::Options {
        parse: markdown::ParseOptions::gfm(),
        compile: markdown::CompileOptions {
            allow_dangerous_html: true,
            ..markdown::CompileOptions::gfm()
        },
    };
    markdown::to_html_with_options(md, &options).map_err(|e| anyhow!(e))
}
