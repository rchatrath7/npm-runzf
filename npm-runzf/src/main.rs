use glob::glob;
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct Command {
    name: String,
    workspace: Option<String>,
}

fn find_root_dir() -> Option<PathBuf> {
    let mut current = std::env::current_dir().ok()?;
    loop {
        if current.join("package.json").is_file() {
            return Some(current);
        }
        if !current.pop() {
            return None;
        }
    }
}

fn get_workspace_patterns(root: &Path) -> Vec<String> {
    let content = match fs::read_to_string(root.join("package.json")) {
        Ok(content) => content,
        Err(_) => return vec![],
    };

    let json: Value = match serde_json::from_str(&content) {
        Ok(json) => json,
        Err(_) => return vec![],
    };

    match &json["workspaces"] {
        Value::Array(patterns) => patterns
            .iter()
            .filter_map(|p| p.as_str())
            .map(String::from)
            .collect(),
        _ => vec![],
    }
}

fn find_package_jsons(root: &Path) -> Vec<PathBuf> {
    let mut package_jsons = vec![];

    // Always include root package.json if it exists
    let root_package = root.join("package.json");
    if root_package.exists() {
        package_jsons.push(root_package);
    }

    // Get workspace patterns from root package.json
    let patterns = get_workspace_patterns(root);

    for pattern in patterns {
        let glob_pattern = root.join(&pattern).join("package.json");
        if let Some(pattern_str) = glob_pattern.to_str() {
            if let Ok(matches) = glob(pattern_str) {
                for entry in matches.filter_map(Result::ok) {
                    package_jsons.push(entry);
                }
            }
        }
    }

    package_jsons
}

fn process_package_json(path: &Path) -> Result<Vec<Command>, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let json: Value = serde_json::from_str(&content)?;

    let scripts = match &json["scripts"] {
        Value::Object(scripts) => scripts.keys().cloned().collect(),
        _ => vec![],
    };

    let workspace = path.parent().and_then(|p| {
        let root = find_root_dir().unwrap_or_else(|| PathBuf::from("."));
        if p == root {
            None
        } else {
            Some(
                p.strip_prefix(&root)
                    .unwrap_or(p)
                    .to_string_lossy()
                    .into_owned(),
            )
        }
    });

    Ok(scripts
        .into_iter()
        .map(|name| Command {
            name,
            workspace: workspace.clone(),
        })
        .collect())
}

fn main() -> Result<(), Box<dyn Error>> {
    let root = find_root_dir().ok_or("Could not find root package.json")?;
    let package_jsons = find_package_jsons(&root);

    let mut all_commands = Vec::new();
    for path in package_jsons {
        if let Ok(mut commands) = process_package_json(&path) {
            all_commands.append(&mut commands);
        }
    }

    // const SEAFOAM: &str = "\x1b[38;2;156;207;216m"; // #9CCFD8
    // const DARK_BLUE: &str = "\x1b[38;2;160;120;192m"; // #A078C0
    // const RESET: &str = "\x1b[38;2;235;111;146m"; // #EB6F92 (Rose Pine love/pink)

    // Format for display: add descriptions and color formatting
    // for cmd in all_commands {
    //     match cmd.workspace {
    //         Some(ws) => println!(
    //             "{SEAFOAM}{}{DARK_BLUE}\t{DARK_BLUE}{}{DARK_BLUE}",
    //             cmd.name, ws
    //         ),
    //         None => println!("{SEAFOAM}{}{DARK_BLUE}", cmd.name),
    //     }
    // }
    for cmd in all_commands {
        match cmd.workspace {
            Some(ws) => println!("{}\t{}", cmd.name, ws),
            None => println!("{}", cmd.name),
        }
    }

    Ok(())
}
