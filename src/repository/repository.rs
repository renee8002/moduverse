// src/repository.rs

use std::fs;
use std::path::Path;
use std::io;
use serde::{Serialize, Deserialize};
use crate::staging::StagingArea;

#[derive(Debug)]
pub struct Repository {
    pub path: String,
}

impl Repository {
    pub fn new(path: &str) -> Self {
        Repository { path: path.to_string() }
    }

    pub fn init(&self) -> Result<String, String> {
        let mdv_dir = Path::new(&self.path).join(".mdv");

        // Check if already initialized
        if fs::metadata(&mdv_dir).is_ok() {
            return Ok("Your project has been initialized.".to_string());
        }
        
        // Initialize staging area
        let mut staging_area = StagingArea::new();
        staging_area.push_staging_file(".mdv/staging_area.json");
        staging_area.save_to_json(".mdv/staging_area.json");

        let mut head_file = revision::Head::new(rev: String::new(), branch:"maser".to_string());
        head_file.save_head(".mdv/head.json");
        head_file.save_to_json(".mdv/head.json");

        // Create necessary directories
        fs::create_dir_all(&mdv_dir)
            .map(|_| format!("Initialized successfully!"))
            .map_err(|err| format!("Initialization failed: {}", err))
    }

    pub fn clone(&self, source_path: &str) -> Result<String, String> {
        let source_path = Path::new(source_path);
        let target_path = Path::new(&self.path);

        if !source_path.exists() {
            return Err(format!("Source path '{}' does not exist.", source_path.display()));
        }

        if !target_path.is_absolute() {
            return Err("Target path must be an absolute path.".to_string());
        }

        copy_recursive(source_path, target_path)
            .map(|_| "Repository cloned successfully.".to_string())
            .or_else(|err| Err(format!("Clone failed: {}", err)))
    }

    pub fn pull(&self, source_path: &str) -> Result<String, String> {
        let conflicts_exist = self.detect_conflicts();
        if conflicts_exist {
            return Err("Conflicts detected. Please merge changes before pulling.".to_string());
        }

        let source_path = Path::new(source_path);
        let target_path = Path::new(&self.path);

        copy_recursive(source_path, target_path)
            .map(|_| "Repository pulled successfully.".to_string())
            .or_else(|err| Err(format!("Pull failed: {}", err)))
    }

    pub fn push(&self, target_path: &str) -> Result<String, String> {
        let conflicts_exist = self.detect_conflicts();
        if conflicts_exist {
            return Err("Conflicts detected. Please merge changes before pushing.".to_string());
        }

        let target_path = Path::new(target_path);
        let source_path = Path::new(&self.path);

        copy_recursive(source_path, target_path)
            .map(|_| "Repository pushed successfully.".to_string())
            .or_else(|err| Err(format!("Push failed: {}", err)))
    }

    fn detect_conflicts(&self) -> bool {
        // detect conflicts

        // if no conflict, return false
        false
    }
}

fn copy_recursive(source: &Path, destination: &Path) -> io::Result<()> {
    fs::create_dir_all(destination)?;

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let destination_entry = destination.join(entry.file_name());
        
        if entry.file_type()?.is_dir() {
            copy_recursive(&entry.path(), &destination_entry)?;
        } else {
            fs::copy(entry.path(), destination_entry)?;
        }
    }

    Ok(())
}

