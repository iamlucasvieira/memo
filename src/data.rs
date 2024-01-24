use crate::app;
use anyhow::{anyhow, Context, Result};
use std::io::prelude::*;
use std::io::BufReader;
use std::{collections::HashMap, fmt, fs, path::PathBuf};

/// Memo data contains the content of the memo file.
pub struct MemoData {
    content: HashMap<i32, String>,
}

impl MemoData {
    /// Create a new MemoData
    pub fn new() -> Self {
        MemoData {
            content: HashMap::new(),
        }
    }

    /// parse data from file and return a HashMap
    fn parse(data: String) -> Result<HashMap<i32, String>> {
        data.lines()
            .filter(|line| !line.is_empty())
            .map(|line| vaidate_line(line))
            .collect()
    }
}

/// Implement DataFile trait for MemoData
impl DataFile for MemoData {
    /// Load data from file
    fn load(&mut self, cli_app: &app::AppConfig) -> Result<()> {
        let data = read_file(&cli_app.data_file_path())?;
        self.content = MemoData::parse(data)?;
        Ok(())
    }

    /// Return sorted ids of items in MemoData
    fn sorted_ids(&self) -> Vec<i32> {
        let mut ids: Vec<i32> = self.content.keys().cloned().collect();
        ids.sort();
        ids
    }

    /// Return content of item with id
    fn get(&self, id: i32) -> Option<&String> {
        self.content.get(&id)
    }

    /// Add item to MemoData
    fn add(&mut self, id: i32, name: &str) -> Result<()> {
        if self.content.contains_key(&id) {
            return Err(anyhow!("Id '{}' already exists", id));
        }
        self.content.insert(id, name.to_string());
        Ok(())
    }

    fn remove(&mut self, id: i32) -> Result<()> {
        if !self.content.contains_key(&id) {
            return Err(anyhow!("Id '{}' not found", id));
        }
        self.content.remove(&id);
        Ok(())
    }

    fn as_string(&self) -> Result<String> {
        let mut s = String::new();
        for id in self.sorted_ids() {
            s.push_str(&format!(
                "{}: {}\n",
                id,
                self.get(id)
                    .ok_or_else(|| anyhow!("Id '{}' not found", id))?
            ));
        }
        return Ok(s);
    }
}

/// Implement Display trait for MemoData
impl fmt::Display for MemoData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (id, name) in &self.content {
            writeln!(f, "{}: {}", id, name)?;
        }
        Ok(())
    }
}

/// DataFile trait is used to define the methods that a data file must implement.
pub trait DataFile: fmt::Display {
    fn load(&mut self, app: &app::AppConfig) -> Result<()>;
    fn sorted_ids(&self) -> Vec<i32>;
    fn get(&self, id: i32) -> Option<&String>;
    fn add(&mut self, id: i32, name: &str) -> Result<()>;
    fn remove(&mut self, id: i32) -> Result<()>;
    fn as_string(&self) -> Result<String>;
}

/// Get file path and file name and check if it exists
fn file_exist(file_path: &PathBuf) -> Result<bool> {
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

pub fn write_file(file_path: &PathBuf, content: &str) -> Result<()> {
    file_exist(file_path)?;
    let mut temp_file_path = file_path.clone();
    temp_file_path.set_extension("tmp");
    let mut temp_file = fs::File::create(&temp_file_path)?;
    temp_file.write_all(content.as_bytes())?;

    fs::rename(&temp_file_path, &file_path)?;
    Ok(())
}

// validate a line of file content
fn vaidate_line(line: &str) -> Result<(i32, String)> {
    let mut parts = line.splitn(2, ':');
    let id = parts
        .next()
        .ok_or_else(|| anyhow!("Missing id in line"))?
        .parse::<i32>()
        .with_context(|| format!("Invalid id in line '{}'", line))?;
    let name = parts
        .next()
        .and_then(|n| if n.is_empty() { None } else { Some(n) })
        .ok_or_else(|| anyhow!("Missing content in line '{}'", line))?
        .trim()
        .to_string();
    Ok((id, name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_memo_data_new() {
        let d = MemoData::new();
        assert_eq!(d.content.len(), 0);
    }

    #[test]
    fn test_memo_data_parse() {
        let data = "1: one\n2: two\n3: three\n".to_string();
        let d = MemoData::parse(data).unwrap();
        assert_eq!(d.len(), 3);
    }

    #[test]
    fn test_memo_data_add() {
        let mut d = MemoData::new();
        assert_eq!(d.add(1, "one").is_ok(), true);
        assert_eq!(d.content.len(), 1);
    }

    #[test]
    fn test_memo_data_load() {
        let mut app_config = app::AppConfig::new("memo", "memo.txt");

        let dir = tempdir().unwrap();
        let mut data_dir = dir.path().to_path_buf();
        data_dir.push(app_config.name());
        fs::create_dir_all(&data_dir).unwrap();

        let mut file_dir = data_dir.clone();
        file_dir.push(app_config.data_file());
        let mut file = fs::File::create(&file_dir).unwrap();
        writeln!(file, "1: one\n2: two\n3: three\n").unwrap();

        app_config.data_dir = data_dir.clone();

        let mut d = MemoData::new();
        assert_eq!(d.load(&app_config).is_ok(), true);
    }

    #[test]
    fn test_memo_data_sorted_ids() {
        let mut d = MemoData::new();
        assert_eq!(d.add(2, "two").is_ok(), true);
        assert_eq!(d.add(1, "one").is_ok(), true);
        assert_eq!(d.add(3, "three").is_ok(), true);
        assert_eq!(d.sorted_ids(), vec![1, 2, 3]);
    }

    #[test]
    fn test_memo_data_get() {
        let mut d = MemoData::new();
        assert_eq!(d.add(1, "one").is_ok(), true);
        assert_eq!(d.get(1).unwrap(), "one");
    }

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
