use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime};
use copypasta::{ClipboardContext, ClipboardProvider};
use serde_json::Value;

fn main() {
    let download_dir = dirs::download_dir().expect("❌ Could not find Downloads folder");
    let notebook = find_latest_ipynb(&download_dir).expect("❌ No .ipynb files found");

    println!("📄 Using file: {}", notebook.display());

    let contents = fs::read_to_string(&notebook).expect("❌ Failed to read notebook file");

    let parsed: Value = serde_json::from_str(&contents).expect("❌ Failed to parse JSON");

    let cells = parsed.get("cells")
        .and_then(|v| v.as_array())
        .expect("❌ 'cells' is missing or not an array");

    println!("🧪 Total cells: {}", cells.len());

    let mut blocks = Vec::new();

    for (i, cell) in cells.iter().enumerate() {
        let cell_type = cell.get("cell_type").and_then(|v| v.as_str());
        if cell_type != Some("code") {
            continue;
        }

        if let Some(source_val) = cell.get("source") {
            let code = match source_val {
                Value::String(s) => s.clone(),
                Value::Array(arr) => arr.iter().filter_map(|v| v.as_str()).collect::<String>(),
                _ => String::new(),
            };

            if !code.trim().is_empty() {
                blocks.push(format!("```python\n{}\n```", code.trim()));
            }
        }
    }

    if blocks.is_empty() {
        println!("⚠️ No code blocks found.");
        return;
    }

    let final_output = blocks.join("\n\n");

    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(final_output).expect("❌ Failed to copy to clipboard");

    println!("✅ Copied {} code blocks to clipboard.", blocks.len());
}

fn find_latest_ipynb(dir: &Path) -> Option<PathBuf> {
    fs::read_dir(dir).ok()?
        .filter_map(|e| e.ok())
        .filter(|e| {
            match e.path().extension().and_then(|s| s.to_str()) {
                Some("ipynb") => true,
                _ => false,
            }
        })
        .max_by_key(|e| e.metadata().and_then(|m| m.modified()).unwrap_or(SystemTime::UNIX_EPOCH))
        .map(|e| e.path())
}
