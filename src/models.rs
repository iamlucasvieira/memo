use crate::style;
use anyhow::{anyhow, Context, Result};
use chrono::prelude::*;
use std::collections::HashMap;
use std::str::FromStr;

/// Struct that holds all the data of the application
/// The data is stored in a HashMap where the key is the id of the item and the value is the content.
pub struct MemoData {
    pub contents: HashMap<u32, Content>,
}

/// Stores the content of a 'memo'
/// Includes the text, the date and time
pub struct Content {
    pub text: String,
    pub date_time: NaiveDateTime,
}

impl MemoData {
    /// Create a new MemoData
    pub fn new() -> Self {
        MemoData {
            contents: HashMap::new(),
        }
    }

    /// parse data from file and return a HashMap
    pub fn parse(data: String) -> Result<HashMap<u32, Content>> {
        data.lines()
            .filter(|line| !line.is_empty())
            .map(vaidate_line)
            .collect()
    }

    /// gets the content of an item given its id
    pub fn get(&self, id: u32) -> Option<&Content> {
        self.contents.get(&id)
    }

    /// Returns a vector with the ids of the items sorted
    pub fn sorted_ids(&self) -> Vec<u32> {
        let mut ids: Vec<u32> = self.contents.keys().cloned().collect();
        ids.sort();
        ids
    }

    /// Returns string with contents split by date
    pub fn group_by_date(&self) -> Result<String> {
        let mut result = String::new();
        let mut previous_date =
            NaiveDate::from_ymd_opt(1, 1, 1).with_context(|| "Error creating date NaiveDate")?;
        for id in self.sorted_ids().iter().rev() {
            let content = self
                .contents
                .get(id)
                .with_context(|| format!("No item found for id '{}'", id))?;

            let currrent_date = content.date_time.date();
            let current_time = content.date_time.time();

            if previous_date != currrent_date || result.is_empty() {
                result.push_str(&format!(
                    "\n\n{}",
                    style::str(
                        &currrent_date.format("%A, %B %e, %Y").to_string(),
                        style::Options::Title
                    )
                ));
            }

            let id_and_time = format!("{:0>#2}: {}", id, current_time);
            result.push_str(&format!(
                "\n{} {}",
                style::str(&id_and_time, style::Options::Muted),
                content.text
            ));
            previous_date = currrent_date;
        }
        // Remove empty lines at the beginning of the string
        result = result.trim_start().to_string();
        Ok(result)
    }
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

    #[test]
    fn test_memo_data_new() {
        let d = MemoData::new();
        assert_eq!(d.contents.len(), 0);
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
    fn test_memo_data_get() {
        let mut d = MemoData::new();
        d.contents.insert(
            1,
            Content {
                text: "one".to_string(),
                date_time: NaiveDate::from_ymd_opt(2001, 1, 1)
                    .expect("Error creating date NaiveDate")
                    .and_hms_opt(1, 1, 1)
                    .expect("Error creating time NaiveDateTime"),
            },
        );
        assert_eq!(d.get(1).unwrap().text, "one");
        assert!(d.get(2).is_none());
    }

    #[test]
    fn test_validate_line() {
        let line = "1: 2001-01-01 01:01:01 one";
        let (id, content) = vaidate_line(line).unwrap();
        assert_eq!(id, 1);
        assert_eq!(content.text, "one");
        assert_eq!(
            content.date_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            "2001-01-01 01:01:01"
        );
    }

    #[test]
    fn test_validate_line_invalid() {
        let line_invalid_id = "a: 2001-01-01 01:01:01 one";
        let line_missing_id = ": 2001-01-01 01:01:01 one";
        let line_missing_content = "1: 2001-01-01 01:01:01";
        let line_invalid_date = "1: 2001-01-01-01 01:01:01 one";

        assert!(vaidate_line(line_invalid_id).is_err());
        assert!(vaidate_line(line_missing_id).is_err());
        assert!(vaidate_line(line_missing_content).is_err());
        assert!(vaidate_line(line_invalid_date).is_err());
    }
}
