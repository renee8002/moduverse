use serde::{Serialize, Deserialize};
use chrono::{Local};
use sha2::{Digest, Sha256};
use crate::staging::StagingArea;
use std::path::Path;

#[derive(Serialize,Deserialize,Debug)]
struct RevisionInfo{
    main_parent: Option<String>,  //SHA-256
    branch_parent: Option<String>, //SHA-256
    id: String, // SHA-256
    author: String,
    date: String,
    commit_msg: String,
    related_files: Vec<String>,
}

#[derive(Debug)]
pub struct Revision{
    path: String,
    info: RevisionInfo,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Head{
    revision_id: String,
    branchname: String,
} 

impl Head {
    pub fn new(rev: String, branch: String) -> Head {
        Head {
            revision_id: rev,
            branchname: branch,
        }
    }

    pub fn set_revision_id(&mut self, id: String) {
        self.revision_id = id;
    }

    pub fn set_branch(&mut self, branch: String) {
        self.branchname = branch;
    }

    pub fn save_head(&self, path: &str) {
        let data = serde_json::to_string(&self).unwrap();
        let _ = writeFile(&path, &data);
    }
}

impl Revision{
    pub fn new(
        path: String,
        main_parent: Option<String>,
        branch_parent: Option<String>,
        author: String,
        commit_msg: String,
        related_files:Vec<String>,
    ) -> Revision {
        let current_time = Local::now().to_string();
        let commit_id = Revision::generate_id(&author,&commit_msg,&current_time,related_files.clone());
        let store_path = format!("{}/{}/{}", path,".mdv/rev",commit_id);
        let _ = fs::create_dir_all(&store_path);
        Revision {
            path: store_path,
            info: RevisionInfo {
                main_parent,
                branch_parent,
                id: commit_id,
                author,
                date: current_time,
                commit_msg,
                related_files,
            },
        }
    }

    fn generate_id(author: &String,commit_msg: &String,date: &String,related_files:Vec<String>) -> String{
        Sha256::digest(
            format!("{}{}{}{:?}", author,commit_msg,date,related_files).as_bytes()
        )
        .iter()
        .fold(String::new(), |acc, byte| acc + &format!("{:02x}", byte))
    }

    fn get_id(&self) -> String{
        self.info.id.clone()
    }

    fn get_parent_ids(&self) -> (Option<&String>, Option<&String>) {
        (
            self.info.main_parent.as_ref(),
            self.info.branch_parent.as_ref(),
        )
    }

    pub fn save_revision(&self){
        let data = serde_json::to_string(&self.info).unwrap();
        let store_path = format!("{}/{}",&self.path, "revision.json");
        let _ =writeFile(&store_path,&data);
    }
}

fn folder_exists(path: &str) -> bool {
    fs::metadata(path).map(|metadata| metadata.is_dir()).unwrap_or(false)
}

fn open_json<T>(path: &str) -> T
where
    T: serde::de::DeserializeOwned,
{
    let json_data = readFile(&path).unwrap();
    serde_json::from_str(&json_data).expect("Failed to deserialize JSON")
}

pub fn open_head(path: &str) -> Head {
    open_json(path)
}

pub fn open_revision(path: &str) -> Revision {
    let json_data = open_json(path);
    Revision { path: path.to_string(), info: json_data }
}

pub fn open_staging_area(path: &str) -> StagingArea {
    let tracked_files: Vec<String> = open_json(path);
    StagingArea { tracked_files }
}


fn FileOperation<F>(path: &str, filenames: Vec<&str>, mut operation: F) -> (Vec<String>, Vec<String>) 
    where F: FnMut(&str) -> Result<(), String>,
{
    filenames
        .into_iter()
        .fold(
            (Vec::new(), Vec::new()),
            |(mut success, mut errors), filename| {
                let file_path = format!("{}/{}", path, filename);
                match operation(&file_path) {
                    Ok(()) => success.push(filename.to_string()),
                    Err(err) => errors.push(format!("Failed: {} -> {}", filename, err)),
                }
                (success, errors)
            },
        )
}

pub fn copy_file(from: &str, to: &str, name:&str){
    let to_path = format!("{}/{}", to, name);
    let _ = createFile(&to_path);
    fs::copy(&from, &to_path).expect("failed to copy");
}

fn ResultFormat(success: Vec<String>, errors: Vec<String>, success_message: &str) -> Result<String, String> {
    if errors.is_empty() {
        Ok(format!("{}: {}", success_message, success.join(", ")))
    } else {
        Err(errors.join("\n"))
    }
}

// create: create new files
pub fn create(path: &str, filenames: Vec<&str>) -> Result<String, String>{
    let (suc_msg, err_msg) = FileOperation(path, filenames, |file_path| {
        createFile(file_path).map_err(|err| err.to_string())
    });
    ResultFormat(suc_msg, err_msg, "Successfully created files")
}

// remove: remove specific files from tracking list
pub fn remove(path: &str, filenames: Vec<&str>) -> Result<String, String>{
    let staging_path = format!("{}/{}/{}", path, ".mdv", "staging_area.json");
    let mut staging_area = open_staging_area(&staging_path);

    let (suc_msg, err_msg)= FileOperation(path, filenames, |file_path| {
        if staging_area.get_tracked_files().contains(&file_path.to_string()) {
            staging_area.remove_staging_file(&file_path);
            Ok(())
        } else {
            Err(format!("Didn't match any files."))
        }
    });

    staging_area.save_to_json(&staging_path);
    ResultFormat(suc_msg, err_msg, "Successfully removed files")
}

// add: add specific files that you want to track
pub fn add(path: &str, filenames: Vec<&str>) -> Result<String, String>{
    let staging_path = format!("{}/{}/{}", path, ".mdv", "staging_area.json");
    let mut staging_area = open_staging_area(&staging_path);

    let (suc_msg, err_msg)= FileOperation(path, filenames, |file_path| {
        staging_area.push_staging_file(&file_path);Ok(())
    });

    staging_area.save_to_json(&staging_path);
    ResultFormat(suc_msg, err_msg, "Successfully added files")
}


// // commit changes and create a new revision
pub fn commit(path: &str, filenames: Vec<&str>, msg: &str, author: &str) -> Result<String, String>{
    let staging_path = format!("{}/{}/{}", path, ".mdv", "staging_area.json");
    let mut staging_area = open_staging_area(&staging_path);
    let head_path = format!("{}/{}/{}", path, ".mdv", "head.json");
    let mut head_file = open_head(&head_path);
    
    let mut rev = Revision::new(
        path.to_string(),
        Some(head_file.revision_id.clone()),
        None,
        author.to_string(),
        msg.to_string(),
        Vec::new(),
    );
    
    let (suc_msg, err_msg) = FileOperation(path, filenames, |file_path| {
        if staging_area.get_tracked_files().contains(&file_path.to_string()) {
            let path = Path::new(file_path);
            let filename = path.file_name().unwrap().to_str().unwrap();
            let _ = copy_file(&file_path, &rev.path, filename);
            rev.info.related_files.push(file_path.to_string());
            staging_area.remove_staging_file(&file_path);
            Ok(())
        } else {
            Err(format!("Files {} are not staged", file_path))
        }
    });

    rev.save_revision();
    staging_area.save_to_json(&staging_path);
    head_file.set_revision_id(rev.get_id());
    head_file.save_head(&head_path);

    ResultFormat(suc_msg, err_msg, "Successfully committed files")
}

// head: show the current head
pub fn head(path: &str) -> Result<String, String>{
    let head_path = format!("{}/{}/{}", path, ".mdv", "head.json");
    let head_file = open_head(&head_path);

    if head_file.revision_id.is_empty(){
        Err("Nothing commited yet, the head is empty".to_string())
    } else{
        let rev_path = format!("{}/{}/{}/{}",path, ".mdv/rev", head_file.revision_id.as_str(),"revision.json");
        let rev = open_revision(&rev_path);
        Ok(format!("Commit: {}\nHead -> {}\nAuthor: {}\nDate: {}", head_file.revision_id, head_file.branchname, rev.info.author, rev.info.date))
    }
} 

// cat: inspect a file of a given revision
pub fn cat(path: &str, commit_id: &str,filename: &str) -> Result<String, String>{
    let rev_path = format!("{}/{}/{}/{}",path, ".mdv/rev", commit_id, filename);

    match readFile(&rev_path){
        Ok(content) => Ok(format!("{}: {}", filename, content)),
        Err(err) => Err(format!("Invalid Revision")),
    }
}

// branch: create a new branch and switch to the branch
pub fn branch(path: &str, branchname: &str) -> Result<String, String>{
    let head_path = format!("{}/{}/{}", path, ".mdv", "head.json");
    let mut head_file = open_head(&head_path);
    let rev_path = format!("{}/{}", path, ".mdv/rev");
    // 未commit的报错; 已存在的branchname报错;
    if (!folder_exists(&rev_path)){
        Err(format!("Not a valid object name:{}", head_file.branchname))
    } else{
        head_file.set_branch(branchname.to_string());
        head_file.save_head(&head_path);
        println!("{:?}",head_file);
        Ok(format!("switched to branch:{}",branchname))
    }
}


// //check out a specific revision
pub fn checkout(path: &str, commit_id: &str) -> Result<String, String>{


}

// use std::fs::{self, copy};
// use std::io::{Error, Write, Read};
// pub fn readFile(path: &str) -> Result<String, Error> {
//     // Read the contents of the file at the specified path
//     let mut content = String::new();
//     fs::File::open(path)?.read_to_string(&mut content)?;
//     Ok(content)
// }

// pub fn createFile(path: &str) -> Result<(), Error> {
//     // Create an empty file at the specified path
//     fs::File::create(path)?;
//     Ok(())
// }

// pub fn writeFile(path: &str, content: &str) -> Result<(), Error> {
//     // Write the provided content to the file at the specified path
//     let mut file = fs::File::create(path)?;
//     file.write_all(content.as_bytes())?;
//     Ok(())
// }