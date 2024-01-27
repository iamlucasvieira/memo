use crate::app;
use crate::data::{read_file, DataFile};
use crate::models::{Content, MemoData};
use anyhow::{anyhow, Context, Result};
use chrono::prelude::*;
use std::fmt;

/// Date time format used in Content
pub const DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

/// Implement FromStr trait for Content
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

/// Implement Display trait for Content
impl fmt::Display for Content {
    /// Format Content for display
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            self.date_time.format(DATE_TIME_FORMAT),
            self.text
        )
    }
}

/// Implement Default trait for MemoData
impl Default for MemoData {
    /// Create a new MemoData
    fn default() -> Self {
        Self::new()
    }
}

/// Implement DataFile trait for MemoData
impl DataFile for MemoData {
    /// Load data from file
    fn load(&mut self, cli_app: &app::AppConfig) -> Result<()> {
        let data = read_file(&cli_app.data_file_path())?;
        self.contents = MemoData::parse(data)?;
        Ok(())
    }

    /// Return sorted ids of items in MemoData
    fn sorted_ids(&self) -> Vec<u32> {
        let mut ids: Vec<u32> = self.contents.keys().cloned().collect();
        ids.sort();
        ids
    }

    /// Add item to MemoData
    fn add(&mut self, id: u32, name: &str) -> Result<()> {
        if self.contents.contains_key(&id) {
            return Err(anyhow!("Id '{}' already exists", id));
        }
        let date_time = Local::now().naive_local();
        self.contents.insert(
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
        if !self.contents.contains_key(&id) {
            return Err(anyhow!("Id '{}' not found", id));
        }
        self.contents.remove(&id);
        Ok(())
    }
}

/// Implement Display trait for MemoData
impl fmt::Display for MemoData {
    /// Format MemoData for display
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for id in self.sorted_ids() {
            writeln!(f, "{}: {}", id, self.contents[&id])?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs;
    use std::io::Write;
    use std::str::FromStr;
    use tempfile::tempdir;

    #[test]
    fn test_memo_data_add() {
        let mut d = MemoData::new();
        assert_eq!(d.add(1, "one").is_ok(), true);
        assert_eq!(d.contents.len(), 1);
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
    fn test_memo_data_remove() {
        let mut d = MemoData::new();
        assert_eq!(d.add(1, "one").is_ok(), true);
        assert_eq!(d.remove(1).is_ok(), true);
        assert_eq!(d.contents.len(), 0);
    }

    #[test]
    fn test_memo_data_get() {
        let mut d = MemoData::new();
        assert_eq!(d.add(1, "one").is_ok(), true);
        assert_eq!(d.get(1).expect("Id should exist").text, "one");
    }

    #[test]
    fn test_memo_data_display() {
        let mut d = MemoData::new();
        let data_str = "1: 2021-01-01 01:01:01 one\n2: 2021-01-01 01:01:01 two\n3: 2021-01-01 01:01:01 three\n";
        d.contents = MemoData::parse(data_str.to_string()).expect("Error parsing data");
        assert_eq!(format!("{}", d), data_str);
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

    #[test]
    fn test_content_display() {
        let content = "2021-01-01 01:01:01 one";
        let c = Content::from_str(content).expect("Error creating Content");
        assert_eq!(format!("{}", c), content);
    }
}
