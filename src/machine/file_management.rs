use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub fn create_file(path: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(path)?;
    file.write_all(b"")?;
    Ok(())
}
pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // file.close()?; // This is not needed in Rust, but it's a good habit to close files
    Ok(contents)
}

pub fn write_file(path: &str, content: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn list_files_in_dir(path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let path_str = path.to_str().unwrap();
        files.push(path_str.to_string());
    }
    Ok(files)
}

// pub fn list_files_in_directory(directory_path: &str) -> io::Result<Vec<String>> {
//     let paths = fs::read_dir(directory_path)?
//         .map(|entry| entry.map(|e| e.path().display().to_string()))
//         .collect::<Result<Vec<_>, io::Error>>()?;
//     Ok(paths)
// }


pub fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

// Add more file-related functions as needed
