use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use std::collections::HashMap;
use std::sync::Mutex;

use notify::{Watcher, RecursiveMode, EventKind, RecommendedWatcher, Config};
use copypasta::{ClipboardContext, ClipboardProvider};
use serde_json::Value;
use once_cell::sync::Lazy;
use clap::{Arg, ArgAction, Command};

static PROCESSED: Lazy<Mutex<HashMap<String, SystemTime>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub mod tree;

pub fn run() -> Result<String, String> {
    let cli = Command::new("snacc")
        .version("0.1.0")
        .author("Simon Bouchard <simon.bouchard@gmail.com>")
        .about("Watches or copies Kaggle notebooks automatically")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("watch")
                .about("Watches the Downloads folder and copies any new notebook")
                .arg(Arg::new("keep").long("keep").action(ArgAction::SetTrue).help("Keep the notebook after copying"))
                .arg(Arg::new("cells").long("cells").default_value("code").value_parser(["code", "markdown", "all"]).help("Which cells to copy")),
        )
        .subcommand(
            Command::new("copy")
                .about("Copies the latest downloaded notebook once")
                .arg(Arg::new("keep").long("keep").action(ArgAction::SetTrue).help("Keep the notebook after copying"))
                .arg(Arg::new("cells").long("cells").default_value("code").value_parser(["code", "markdown", "all"]).help("Which cells to copy")),
        )
        .subcommand(
            Command::new("tree")
                .about("Print the directory tree")
                .arg(
                    Arg::new("path")
                        .short('p')
                        .long("path")
                        .help("Path to start from")
                        .default_value("."),
                )
                .arg(
                    Arg::new("no-copy")
                        .long("no-copy")
                        .help("Do not copy the result to the clipboard")
                        .action(ArgAction::SetTrue),
                ),
        )
        .get_matches();

    let download_dir = dirs::download_dir().ok_or("❌ Could not find Downloads folder")?;

    match cli.subcommand() {
        Some(("watch", sub)) => {
            let cell_mode = sub.get_one::<String>("cells").unwrap().to_string();
            let delete_file = !sub.get_flag("keep");

            // Don't log to stdout; watch_loop runs silently
            watch_loop(download_dir, cell_mode, delete_file)
                .map_err(|e| format!("❌ Watch loop failed: {}", e))?;

            Ok("👀 Watch loop ended (unexpectedly)".to_string())
        }

        Some(("copy", sub)) => {
            let cell_mode = sub.get_one::<String>("cells").unwrap().to_string();
            let delete_file = !sub.get_flag("keep");

            if let Some(latest) = get_latest_ipynb(&download_dir) {
                handle_notebook(latest, &cell_mode, delete_file)
            } else {
                Err("❌ No .ipynb file found.".to_string())
            }
        }

        Some(("tree", sub)) => {
            let path = sub.get_one::<String>("path").unwrap();
            let copy = !sub.get_flag("no-copy");
            tree::generate_tree(path, copy);
            Ok("".to_string())
        }

        _ => unreachable!(),
    }
}

pub fn watch_loop(download_dir: PathBuf, cell_mode: String, delete_file: bool) -> notify::Result<()> {
    let mut watcher = RecommendedWatcher::new(
        move |res: Result<notify::Event, notify::Error>| {
            if let Ok(event) = res {
                if let Some(path) = event.paths.first() {
                    if matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_)) {
                        if path.extension().and_then(|s| s.to_str()) == Some("ipynb") {
                            let Ok(meta) = fs::metadata(path) else { return; };
                            let Ok(modified) = meta.modified() else { return; };

                            let key = path.to_string_lossy().to_string();
                            let mut seen = PROCESSED.lock().unwrap();

                            if let Some(prev_time) = seen.get(&key) {
                                if *prev_time == modified {
                                    return; // Already handled this exact version
                                }
                            }

                            seen.insert(key, modified);

                            // Quietly discard the result
                            let _ = handle_notebook(path.clone(), &cell_mode, delete_file);
                        }
                    }
                }
            }
        },
        Config::default(),
    )?;

    watcher.watch(&download_dir, RecursiveMode::NonRecursive)?;

    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}

pub fn handle_notebook(path: PathBuf, cell_mode: &str, delete_file: bool) -> Result<String, String> {
    std::thread::sleep(Duration::from_secs(1));

    let contents = fs::read_to_string(&path)
        .map_err(|e| format!("❌ Failed to read: {}", e))?;

    let parsed: Value = serde_json::from_str(&contents)
        .map_err(|e| format!("❌ Failed to parse JSON: {}", e))?;

    let cells = parsed.get("cells")
        .and_then(|v| v.as_array())
        .ok_or("⚠️ No 'cells' array found.")?;

    let mut blocks = Vec::new();

    for cell in cells {
        let typ = cell.get("cell_type").and_then(|v| v.as_str());
        if !matches!(cell_mode, "all") && typ != Some(cell_mode) {
            continue;
        }

        if let Some(source_val) = cell.get("source") {
            let content = match source_val {
                Value::String(s) => s.clone(),
                Value::Array(arr) => arr.iter().filter_map(|v| v.as_str()).collect::<String>(),
                _ => String::new(),
            };

            if !content.trim().is_empty() {
                let lang = if typ == Some("code") { "python" } else { "" };
                blocks.push(format!("```{}\n{}\n```", lang, content.trim()));
            }
        }
    }

    if blocks.is_empty() {
        return Err(format!("⚠️ No {} blocks found.", cell_mode));
    }

    let final_output = blocks.join("\n\n");

    let mut ctx = ClipboardContext::new().map_err(|e| {
        format!(
            "❌ Failed to access clipboard: {}{}\n",
            e,
            if cfg!(target_os = "linux") {
                "\n💡 Tip: On Linux, try installing `xclip`, `xsel`, or `wl-clipboard` (e.g., `sudo apt install xclip`)."
            } else {
                ""
            }
        )
    })?;

    ctx.set_contents(final_output).map_err(|e| format!("❌ Failed to copy to clipboard: {}", e))?;

    let mut msg = format!("✅ Copied {} {} cells to clipboard.", blocks.len(), cell_mode);

    if delete_file {
        std::thread::sleep(Duration::from_millis(500));
        match fs::remove_file(&path) {
            Ok(_) => {
                msg.push_str(&format!("\n🗑️ Deleted: {}", path.display()));
            }
            Err(e) => {
                msg.push_str(&format!("\n⚠️ Could not delete file: {}", e));
            }
        }
    }

    Ok(msg)
}

pub fn get_latest_ipynb(download_dir: &Path) -> Option<PathBuf> {
    let mut files: Vec<_> = fs::read_dir(download_dir)
        .ok()?
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().and_then(|ext| ext.to_str()) == Some("ipynb"))
        .collect();

    files.sort_by_key(|e| e.metadata().and_then(|m| m.modified()).unwrap_or(SystemTime::UNIX_EPOCH));
    files.pop().map(|e| e.path())
}
