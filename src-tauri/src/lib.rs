use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ScanFile {
  virtual_path: String,
  abs_path: String,
  category: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ScanResult {
  root: String,
  label: String,
  files: Vec<ScanFile>,
}

fn categorize_file(path: &Path) -> Option<&'static str> {
  let name_lower = path.file_name()?.to_string_lossy().to_lowercase();
  if name_lower.ends_with(".mm.md") {
    return Some("mindmap");
  }
  if name_lower.ends_with(".ppt.md") {
    return Some("marpit");
  }

  let ext = path.extension()?.to_string_lossy().to_lowercase();
  match ext.as_str() {
    "png" | "jpg" | "jpeg" | "gif" | "webp" => Some("images"),
    "mp4" | "webm" | "ogv" | "m4v" => Some("video"),
    "mp3" | "wav" | "m4a" | "ogg" | "oga" | "flac" | "aac" => Some("audio"),
    "md" | "markdown" => Some("markdown"),
    "drawio" => Some("drawio"),
    "pdf" => Some("pdf"),
    "docx" => Some("word"),
    "xlsx" => Some("excel"),
    "txt" => Some("text"),
    "pptx" => Some("slides"),
    _ => None,
  }
}

fn scan_supported_files(root: &Path) -> Vec<ScanFile> {
  let mut stack: Vec<PathBuf> = vec![root.to_path_buf()];
  let mut files = Vec::new();

  while let Some(dir) = stack.pop() {
    let entries = match std::fs::read_dir(&dir) {
      Ok(entries) => entries,
      Err(_) => continue,
    };

    for entry in entries {
      let entry = match entry {
        Ok(entry) => entry,
        Err(_) => continue,
      };

      let file_type = match entry.file_type() {
        Ok(file_type) => file_type,
        Err(_) => continue,
      };

      let path = entry.path();
      if file_type.is_dir() {
        stack.push(path);
        continue;
      }
      if !file_type.is_file() {
        continue;
      }

      let Some(category) = categorize_file(&path) else {
        continue;
      };

      let rel = match path.strip_prefix(root) {
        Ok(rel) => rel,
        Err(_) => continue,
      };

      files.push(ScanFile {
        virtual_path: rel.to_string_lossy().replace('\\', "/"),
        abs_path: path.to_string_lossy().into_owned(),
        category: category.to_string(),
      });
    }
  }

  files.sort_by(|a, b| a.virtual_path.cmp(&b.virtual_path));
  files
}

#[tauri::command]
fn pick_and_scan_folder() -> Result<Option<ScanResult>, String> {
  let Some(root) = rfd::FileDialog::new().pick_folder() else {
    return Ok(None);
  };
  if !root.is_dir() {
    return Err("选择的路径不是文件夹".to_string());
  }

  let label = root
    .file_name()
    .map(|name| name.to_string_lossy().into_owned())
    .unwrap_or_else(|| root.display().to_string());

  Ok(Some(ScanResult {
    root: root.to_string_lossy().into_owned(),
    label,
    files: scan_supported_files(&root),
  }))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![pick_and_scan_folder])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
