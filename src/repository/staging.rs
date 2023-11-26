pub struct StagingArea{
    pub tracked_files: Vec<String>,  //path
}

impl StagingArea{
    pub fn new() -> Self{
        Self{
            tracked_files: Vec::new(),
        }
    }

    pub fn save_to_json(&self, file_path: &str){
        let data = serde_json::to_string(&self.tracked_files).unwrap();
        let _ =writeFile(&file_path,&data);
    }

    pub fn remove_staging_file(&mut self, file_path: &str) {
        self.tracked_files.retain(|x| x!= file_path)
    }

    pub fn push_staging_file(&mut self, file_path: &str) {
        if !self.tracked_files.contains(&file_path.to_string()){
            self.tracked_files.push(file_path.to_string());
        }
    }

    pub fn get_tracked_files(&self) -> &Vec<String> {
        &self.tracked_files
    }

    pub fn staging_is_empty(&self) -> bool{
        self.tracked_files.is_empty()
    }

    pub fn clear_staging(&mut self){
        self.tracked_files.clear();
    }
}

use std::fs;
use std::io::{Error, Write, Read};
pub fn writeFile(path: &str, content: &str) -> Result<(), Error> {
    // Write the provided content to the file at the specified path
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}