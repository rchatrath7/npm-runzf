use glob::glob;
use serde_json::{Map, Value};
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

    let root_package = root.join("package.json");
    if root_package.exists() {
        package_jsons.push(root_package);
    }

    let patterns = get_workspace_patterns(root);

    //  for pattern in patterns {
    //     let glob_pattern = root.join(pattern).join("package.json");
    //     if let Ok(pattern_str) = glob_pattern.to_str() {
    //         if let Ok(matches) = glob(pattern_str) {
    //             for entry in matches.filter_map(Result::ok) {
    //                 package_jsons.push(entry);
    //             }
    //         }
    //     }
    // }
    //
    // println!("{:?}", patterns);

    for pattern in patterns {
        let glob_pattern = root.join(pattern).join("package.json");
        let pattern_str = glob_pattern.to_str().unwrap();
        let matches = glob(pattern_str).unwrap();
        for entry in matches.filter_map(Result::ok) {
            package_jsons.push(entry);
        }
    }

    package_jsons
}

fn extract_scripts(obj: &Map<String, Value>) -> Vec<String> {
    match obj.get("scripts") {
        Some(scripts) => scripts
            .as_object()
            .map(|scripts| scripts.keys().map(|k| k.to_string()).collect())
            .unwrap(),
        None => return vec![],
    }
}

fn process_package_json(path: &Path) -> Result<Vec<Command>, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let json: Value = serde_json::from_str(&content)?;

    let scripts = match json.as_object() {
        Some(obj) => extract_scripts(obj),
        None => return Ok(vec![]),
    };

    let workspace = path.parent().and_then(|p| {
        if p == Path::new(".") {
            None
        } else {
            Some(p.to_string_lossy().into_owned())
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
    let args: Vec<String> = std::env::args().collect();
    let mut workspace_filter = None;

    for i in 0..args.len() {
        if args[i] == "--workspace" && i + 1 < args.len() {
            workspace_filter = Some(args[i + 1].clone());
            break;
        }
    }

    let root = find_root_dir().ok_or("Could not find root package.json")?;
    let package_jsons = find_package_jsons(&root);

    let mut all_commands = Vec::new();
    for path in package_jsons {
        if let Ok(mut commands) = process_package_json(&path) {
            all_commands.append(&mut commands);
        }
    }

    let filtered_commands: Vec<&Command> = match &workspace_filter {
        Some(ws) => all_commands
            .iter()
            .filter(|cmd| cmd.workspace.as_ref().map_or(false, |w| w.contains(ws)))
            .collect(),
        None => all_commands.iter().collect(),
    };

    for cmd in filtered_commands {
        match &cmd.workspace {
            Some(ws) => println!(
                "{} -w {}",
                cmd.name,
                Path::new(ws)
                    .strip_prefix(root.to_str().unwrap())
                    .unwrap()
                    .to_str()
                    .unwrap()
            ),
            None => println!("{}", cmd.name),
        }
    }

    Ok(())
}
