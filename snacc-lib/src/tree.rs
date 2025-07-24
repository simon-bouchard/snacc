use std::fs;
use std::path::{Path, PathBuf};

const EXCLUDED_NAMES: &[&str] = &[
    // Version control
    ".git", ".hg", ".svn", ".gitattributes", ".gitignore",

    // Rust & build
    "target", "build", "dist", "out",

    // Python
    "__pycache__", ".mypy_cache", ".venv", "venv",

    // Node
    "node_modules",

    // Editors & IDEs
    ".idea", ".vscode", ".DS_Store",

    // Windows/WSL junk
    "Thumbs.db", "ehthumbs.db", "desktop.ini", "Icon?", "$RECYCLE.BIN", "System Volume Information", "Zone.Identifier",

    // Misc
    ".cache",
];

const EXCLUDED_SUFFIXES: &[&str] = &[
    ".log", ".tmp", ".swp", ".bak", ".old", ".orig", // temp/edit files
    ".pdb", ".ilk", ".lnk",                          // Windows debug/symlink
    ".exe", ".dll", ".class", ".pyc",                // binaries
];

pub fn build_tree(root: &Path, prefix: String) -> Vec<String> {
    let mut lines = Vec::new();

    let Ok(entries) = fs::read_dir(root) else {
        lines.push(format!("Error reading: {}", root.display()));
        return lines;
    };

    let mut entries: Vec<_> = entries
        .filter_map(Result::ok)
        .filter(|e| {
            if let Some(name) = e.file_name().to_str() {
                // Skip hidden files
                if name.starts_with('.') {
                    return false;
                }

                // Skip exact name matches
                if EXCLUDED_NAMES.contains(&name) {
                    return false;
                }

                // Skip by suffix
                if let Some(ext) = name.rsplit_once('.') {
                    if EXCLUDED_SUFFIXES.iter().any(|suf| name.ends_with(suf)) {
                        return false;
                    }
                }

                true
            } else {
                false
            }
        })
        .collect();

    entries.sort_by_key(|e| e.file_name());

    for (i, entry) in entries.iter().enumerate() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        let is_last = i == entries.len() - 1;
        let connector = if is_last { "└── " } else { "├── " };
        lines.push(format!("{}{}{}", prefix, connector, name));

        if path.is_dir() {
            let new_prefix = if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };
            lines.extend(build_tree(&path, new_prefix));
        }
    }

    lines
}

pub fn get_tree_string(path: &str) -> String {
    let root = PathBuf::from(path);
    let mut lines = vec![format!("📁 Project tree from: {}", root.display()), String::new()];
    lines.extend(build_tree(&root, String::new()));
    lines.join("\n")
}

pub fn generate_tree(path: &str, copy: bool) {
    let result = get_tree_string(path);

    if copy {
        use copypasta::{ClipboardContext, ClipboardProvider};
        if let Ok(mut ctx) = ClipboardContext::new() {
            let _ = ctx.set_contents(result);
            println!("✅ Project structure copied to clipboard.");
        }
    }
}

