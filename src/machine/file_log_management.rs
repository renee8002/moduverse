// use std::fs::OpenOptions;
// use std::io::Write;
// use std::time::SystemTime;
//
// pub fn log_change(file_path: &str, change_type: &str) -> Result<(), std::io::Error>{
//     // Implement logging of file changes
//     let mut log_file = OpenOptions::new()
//         .append(true)
//         .create(true)
//         .open("log.txt")
//         .unwrap();
//     let timestamp = SystemTime::now()
//         .duration_since(SystemTime::UNIX_EPOCH)
//         .unwrap()
//         .as_secs();
//     let log_entry = format!("{} {} {}\n", timestamp, file_path, change_type);
//     log_file.write_all(log_entry.as_bytes()).unwrap();
//     Ok(())
// }
//
// // Additional logging functions

use serde_json;
use std::fs::{self}; //, File
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use sha2::{Sha256, Digest};


use std::collections::HashMap;

// #[derive(Serialize, Deserialize, Debug)]
pub struct ObjectId(String); // Assuming ObjectId is a wrapper around a String

// #[derive(Serialize, Deserialize, Debug)]
pub struct Directory {
    files: HashMap<String, ObjectId>,
}


// Assuming SnapShot and other relevant structs are defined here or imported
// #[derive(Serialize, Deserialize, Debug)]
pub struct SnapShot {
    message: String,
    directory: ObjectId,
    previous: Option<ObjectId>,
}

pub struct FileLogManager {
    object_store_path: PathBuf,
    branches_dir: PathBuf,
    current_branch: String,
}

impl FileLogManager {
    pub fn new(repo_path: &Path) -> Self {
        let object_store_path = repo_path.join(".rev/store");
        let branches_dir = repo_path.join(".rev/branches");
        let current_branch_path = repo_path.join(".rev/branch");
        let current_branch = fs::read_to_string(current_branch_path)
            .unwrap_or_else(|_| "main".to_string()); // Default to 'main' branch

        FileLogManager {
            object_store_path,
            branches_dir,
            current_branch,
        }
    }

    pub fn log_snapshot(&self, snapshot: &SnapShot) -> io::Result<()> {
        let snapshot_json = serde_json::to_string_pretty(snapshot)?;
        let snapshot_id = self.generate_object_id(&snapshot_json); // Implement this method based on your hash function
        let snapshot_path = self.object_store_path.join(&snapshot_id);

        fs::write(snapshot_path, snapshot_json)?;

        let branch_file_path = self.branches_dir.join(&self.current_branch);
        fs::write(branch_file_path, snapshot_id)?;

        Ok(())
    }

    pub fn get_latest_snapshot(&self) -> io::Result<SnapShot> {
        let branch_file_path = self.branches_dir.join(&self.current_branch);
        let snapshot_id = fs::read_to_string(branch_file_path)?;

        let snapshot_path = self.object_store_path.join(snapshot_id);
        let snapshot_json = fs::read_to_string(snapshot_path)?;
        let snapshot: SnapShot = serde_json::from_str(&snapshot_json)?;

        Ok(snapshot)
    }

    pub fn generate_object_id(&self, content: &str) -> ObjectId {
        let mut hasher = Sha256::new();
        hasher.update(content);
        let result = hasher.finalize();
        ObjectId(format!("{:x}", result))
        // ObjectId(hex::encode(result))

    }
    // pub fn revert_to_snapshot(&self, snapshot_id: ObjectId) -> io::Result<()> {
    //     let snapshot = self.get_snapshot_by_id(&snapshot_id)?;
    //     // Logic to revert the repository to the state described by `snapshot`
    //     // This might involve checking out files, resetting directory states, etc.
    //
    //     Ok(())
    // }

    fn get_snapshot_by_id(&self, snapshot_id: &ObjectId) -> io::Result<SnapShot> {
        let snapshot_path = self.object_store_path.join(&snapshot_id.0);
        let snapshot_json = fs::read_to_string(snapshot_path)?;
        let snapshot: SnapShot = serde_json::from_str(&snapshot_json)?;

        Ok(snapshot)
    }

    // pub fn get_snapshot_history(&self) -> io::Result<Vec<SnapShot>> {
        // let mut history = Vec::new();
        // let mut current_snapshot = self.get_latest_snapshot()?;

        // while let Some(prev_id) = current_snapshot.previous {
        //     history.push(current_snapshot); // Used moved value
        //     current_snapshot = self.get_snapshot_by_id(&prev_id)?;
        // }

        // history.push(current_snapshot); // Add the initial snapshot
        // Ok(history)
        // to_do!("Implement this method")
    // }
    // Additional methods like revert_to_snapshot, get_snapshot_history, etc.
}
