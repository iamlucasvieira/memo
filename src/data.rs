use crate::app;
use anyhow::{anyhow, Result};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::{fmt, fs, path::PathBuf};

pub enum DisplayMode {
    Sorted,
    GroupByDate,
}

/// DataFile trait is used to define the methods that a data file must implement.
pub trait DataFile: fmt::Display {
    fn load(&mut self, app: &app::AppConfig) -> Result<()>;
    fn sorted_ids(&self) -> Vec<u32>;
    fn add(&mut self, id: u32, name: &str) -> Result<()>;
    fn remove(&mut self, id: u32) -> Result<()>;
    fn display(&self, mode: DisplayMode) -> Result<()>;
}

/// Get file path and file name and check if it exists
fn file_exist(file_path: &Path) -> Result<bool> {
    if file_path.exists() {
        Ok(true)
    } else {
        Err(anyhow!(
            "File '{}' not found",
            file_path.to_str().unwrap_or("unknown path")
        ))
    }
}

/// Read the content of a file given its path and name
pub fn read_file(file_path: &PathBuf) -> Result<String> {
    file_exist(file_path)?;
    let mut buff_reader = BufReader::new(fs::File::open(file_path)?);
    let mut contents = String::new();
    buff_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Write content to a file given its path and name
pub fn write_file(file_path: &PathBuf, content: &str) -> Result<()> {
    file_exist(file_path)?;
    let mut temp_file_path = file_path.clone();
    temp_file_path.set_extension("tmp");
    let mut temp_file = fs::File::create(&temp_file_path)?;
    temp_file.write_all(content.as_bytes())?;

    fs::rename(&temp_file_path, file_path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_file_exist() {
        let file_name = "test.txt";
        let dir = tempdir().unwrap();
        let mut file_path = dir.path().to_path_buf();
        file_path.push(file_name);
        let mut file = fs::File::create(&file_path).unwrap();
        writeln!(file, "test").unwrap();
        assert_eq!(file_exist(&file_path).unwrap(), true);
    }

    #[test]
    fn test_read_file() {
        let file_name = "test.txt";
        let dir = tempdir().unwrap();
        let mut file_path = dir.path().to_path_buf();
        file_path.push(file_name);
        let mut file = fs::File::create(&file_path).unwrap();
        writeln!(file, "test").unwrap();
        assert_eq!(read_file(&file_path).unwrap(), "test\n");
    }

    #[test]
    fn test_write_file() {
        let file_name = "test.txt";
        let dir = tempdir().unwrap();
        let mut file_path = dir.path().to_path_buf();
        file_path.push(file_name);
        let mut file = fs::File::create(&file_path).unwrap();
        writeln!(file, "test").unwrap();
        assert_eq!(read_file(&file_path).unwrap(), "test\n");

        let content = "test2\n";
        write_file(&file_path, content).unwrap();
        assert_eq!(read_file(&file_path).unwrap(), content);
    }
}
