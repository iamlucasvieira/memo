use crate::app;
use anyhow::{anyhow, Context, Result};
use chrono::prelude::*;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;
use std::{collections::HashMap, fmt, fs, path::PathBuf};

const DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

/// Struct that holds all the data of the application
/// The data is stored in a HashMap where the key is the id of the item and the value is the content.
pub struct MemoData {
    content: HashMap<u32, Content>,
}

/// Stores the content of a 'memo'
/// Includes the text, the date and time
pub struct Content {
    pub text: String,
    pub date_time: NaiveDateTime,
}

impl std::str::FromStr for Content {
    type Err = anyhow::Error;

    /// Create a Content struct from a string
    /// String format: %Y-%m-%d %H:%M:%S content
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<_> = s.trim().splitn(3, ' ').collect();

        if parts.len() != 3 {
            return Err(anyhow!(
                "Invalid content '{}'.\nExpected: format: %Y-%m-%d %H:%M:%S content",
                s
            ));
        }

        // Turn date time into NaiveDateTime
        let date_time = format!("{} {}", parts[0], parts[1]);
        let date_time = NaiveDateTime::parse_from_str(&date_time, DATE_TIME_FORMAT)
            .with_context(|| format!("invalid date time '{}'", date_time))?;

        let content = parts[2];

        Ok(Content {
            text: content.to_string(),
            date_time,
        })
    }
}
impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            self.date_time.format(DATE_TIME_FORMAT),
            self.text
        )
    }
}

impl MemoData {
    /// Create a new MemoData
    pub fn new() -> Self {
        MemoData {
            content: HashMap::new(),
        }
    }

    /// parse data from file and return a HashMap
    fn parse(data: String) -> Result<HashMap<u32, Content>> {
        data.lines()
            .filter(|line| !line.is_empty())
            .map(vaidate_line)
            .collect()
    }

    /// gets the content of an item given its id
    pub fn get(&self, id: u32) -> Option<&Content> {
        self.content.get(&id)
    }
}

/// Implement Default trait for MemoData
impl Default for MemoData {
    fn default() -> Self {
        Self::new()
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
    fn sorted_ids(&self) -> Vec<u32> {
        let mut ids: Vec<u32> = self.content.keys().cloned().collect();
        ids.sort();
        ids
    }

    /// Add item to MemoData
    fn add(&mut self, id: u32, name: &str) -> Result<()> {
        if self.content.contains_key(&id) {
            return Err(anyhow!("Id '{}' already exists", id));
        }
        let date_time = Local::now().naive_local();
        self.content.insert(
            id,
            Content {
                text: name.to_string(),
                date_time,
            },
        );
        Ok(())
    }

    /// Remove item from MemoData
    fn remove(&mut self, id: u32) -> Result<()> {
        if !self.content.contains_key(&id) {
            return Err(anyhow!("Id '{}' not found", id));
        }
        self.content.remove(&id);
        Ok(())
    }
}

/// Implement Display trait for MemoData
impl fmt::Display for MemoData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for id in self.sorted_ids() {
            writeln!(f, "{}: {}", id, self.content[&id])?;
        }
        Ok(())
    }
}

/// DataFile trait is used to define the methods that a data file must implement.
pub trait DataFile: fmt::Display {
    fn load(&mut self, app: &app::AppConfig) -> Result<()>;
    fn sorted_ids(&self) -> Vec<u32>;
    fn add(&mut self, id: u32, name: &str) -> Result<()>;
    fn remove(&mut self, id: u32) -> Result<()>;
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

/// validate a line of file content
/// Each line must have the format: id: [yyyy-mm-dd hh:mm:ss] content
fn vaidate_line(line: &str) -> Result<(u32, Content)> {
    let mut parts = line.splitn(2, ':');
    let id = parts
        .next()
        .ok_or_else(|| anyhow!("Missing id in line"))?
        .parse::<u32>()
        .with_context(|| format!("Invalid id in line '{}'", line))?;
    let content = parts
        .next()
        .ok_or_else(|| anyhow!("Missing content in line"))?
        .trim();
    let content = Content::from_str(content)?;
    Ok((id, content))
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
        let data =
            "1: 2001-01-01 01:01:01 one\n2: 2002-02-02 02:02:02 two\n3: 2003-03-03 03:03:03 three\n"
                .to_string();
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
        writeln!(file, "1: 2001-01-01 01:01:01 one\n2: 2002-02-02 02:02:02 two\n3: 2003-03-03 03:03:03 three\n").unwrap();

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
        assert_eq!(d.get(1).expect("Id should exist").text, "one");
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

    #[test]
    fn test_content_from_str() {
        let content = "2021-01-01 01:01:01 one";
        let date_time = "2021-01-01 01:01:01";
        let c = Content::from_str(content).expect("Error creating Content");
        assert_eq!(c.text, "one");
        assert_eq!(c.date_time.format(DATE_TIME_FORMAT).to_string(), date_time);
    }

    #[test]
    fn test_content_from_str_spaces() {
        let content = " 2021-01-01 01:01:01 one two three ";
        let date_time = "2021-01-01 01:01:01";
        let c = Content::from_str(content).expect("Error creating Content");
        assert_eq!(c.text, "one two three");
        assert_eq!(c.date_time.format(DATE_TIME_FORMAT).to_string(), date_time);
    }
}
