use notify::{Watcher, RecursiveMode, Config};
use std::fs;
use std::path::Path;

fn main() {
    let path_to_watch = "./watch_me";
    
    // Create the watch folder and subfolders if they don't exist
    let folders = ["Documents", "Images", "Archives", "Others"];
    for folder in folders {
        fs::create_dir_all(format!("{}/{}", path_to_watch, folder)).unwrap();
    }

    println!("Sentinel is watching: {}", path_to_watch);

    // Set up the watcher
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = notify::RecommendedWatcher::new(tx, Config::default()).unwrap();
    watcher.watch(Path::new(path_to_watch), RecursiveMode::NonRecursive).unwrap();

    for res in rx {
        match res {
            Ok(event) => {
                if let notify::EventKind::Create(_) = event.kind {
                    for path in event.paths {
                        organize_file(&path);
                    }
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn organize_file(path: &Path) {
    if path.is_dir() { return; } // Ignore folders

    let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    let destination = match extension.to_lowercase().as_str() {
        "pdf" | "doc" | "docx" | "txt" => "Documents",
        "jpg" | "png" | "gif" | "svg" => "Images",
        "zip" | "rar" | "7z" | "tar" => "Archives",
        _ => "Others",
    };

    let file_name = path.file_name().unwrap();
    let dest_path = path.parent().unwrap().join(destination).join(file_name);

    if let Err(e) = fs::rename(path, dest_path) {
        println!("Error moving file: {:?}", e);
    } else {
        println!("Moved {:?} to {}", file_name, destination);
    }
}