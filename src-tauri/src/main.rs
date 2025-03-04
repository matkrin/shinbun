// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    io::{self, Read},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use clap::Parser;
use comrak::markdown_to_html;
use notify::{
    event::{AccessKind, AccessMode},
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

fn read_stdin() -> Option<String> {
    if atty::is(atty::Stream::Stdin)
        && atty::is(atty::Stream::Stderr)
        && atty::is(atty::Stream::Stdout)
    {
        return None;
    }
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    Some(buffer)
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
    let state = state.lock().unwrap();
    state.sync
}

#[tauri::command]
fn is_stdin(state: tauri::State<Arc<Mutex<MdState>>>) -> bool {
    let state = state.lock().unwrap();
    state.stdin.is_some()
}

#[tauri::command]
fn load_markdown(state: tauri::State<Arc<Mutex<MdState>>>) -> Option<String> {
    let state = state.lock().unwrap();
    if let Some(md) = &state.stdin {
        Some(read_markdown(md.to_string()))
    } else {
        state.md_file.as_ref().map(|x| read_markdown_from_file(x))
    }
}

#[tauri::command]
fn watch_file(window: tauri::Window, state: tauri::State<Arc<Mutex<MdState>>>) {
    let state_clone = Arc::clone(&state);
    let win = window.clone();
    std::thread::spawn(move || {
        let state = state_clone.lock().unwrap();
        let md_file = if let Some(f) = &state.md_file {
            f
        } else {
            return;
        };

        let (tx, rx) = std::sync::mpsc::channel();

        let mut watcher = notify::recommended_watcher(tx).unwrap();
        watcher
            .watch(md_file, notify::RecursiveMode::NonRecursive)
            .unwrap();
        loop {
            match rx.recv() {
                Ok(event) => {
                    if let Ok(e) = &event {
                        if let notify::EventKind::Access(AccessKind::Close(AccessMode::Write)) =
                            e.kind
                        {
                            let md = read_markdown_from_file(md_file);
                            win.emit("watch", &md).unwrap();
                        }
                    };
                }
                Err(e) => eprintln!("Watch error: {:?}", e),
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
        let md = buffer
            .as_bytes()
            .split(|&it| it == 0)
            .map(|it| std::str::from_utf8(it).unwrap())
            .collect::<Vec<_>>()
            .join("\n");
        win.emit("sync", read_markdown(md)).unwrap();
        // std::thread::sleep(Duration::from_millis(10));
    });
}

#[tauri::command]
fn exit(app_handle: tauri::AppHandle) {
    app_handle.exit(1);
}

fn read_markdown_from_file(md_file: &Path) -> String {
    let md = std::fs::read_to_string(md_file).unwrap_or("".to_string());
    read_markdown(md)
}

fn read_markdown(md: String) -> String {
    let mut options = comrak::Options::default();
    options.extension.strikethrough = true;
    options.extension.tagfilter = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.superscript = true;
    options.extension.header_ids = Some("user-content-".to_string());
    options.extension.footnotes = true;
    options.extension.description_lists = true;
    options.extension.front_matter_delimiter = Some("---".to_string());

    options.render.unsafe_ = true;

    markdown_to_html(&md, &options)
}
