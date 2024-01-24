use anyhow::{anyhow, Context, Result};
use std::path::PathBuf;

use crate::data;

pub struct AppConfig {
    name: String,
    data_file: String,
    pub data_dir: PathBuf,
}

impl AppConfig {
    pub fn new(name: &str, data_file: &str) -> Self {
        let system_data_dir = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));

        AppConfig {
            name: name.to_string(),
            data_file: data_file.to_string(),
            data_dir: system_data_dir.join(name),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn data_file(&self) -> &str {
        &self.data_file
    }

    pub fn data_file_path(&self) -> PathBuf {
        let mut path = self.data_dir.clone();
        path.push(&self.data_file);
        path
    }

    pub fn data_dir(&self) -> &PathBuf {
        &self.data_dir
    }
}

pub fn list(d: &Box<dyn data::DataFile>) -> Result<()> {
    d.sorted_ids().iter().for_each(|id| {
        println!("{}: {}", id, d.get(*id).unwrap());
    });
    Ok(())
}

pub fn init(app_config: &AppConfig) -> Result<()> {
    // Check if file exist
    if app_config.data_file_path().exists() {
        return Err(anyhow!(
            "File '{}' already exists",
            app_config.data_file_path().display()
        ));
    }

    // Create directory
    std::fs::create_dir_all(&app_config.data_dir).with_context(|| {
        format!(
            "Could not create directory '{}'",
            app_config.data_dir.display()
        )
    })?;

    // Create file
    std::fs::File::create(&app_config.data_file_path()).with_context(|| {
        format!(
            "Could not create file '{}'",
            app_config.data_file_path().display()
        )
    })?;
    Ok(())
}

pub fn add(d: &mut Box<dyn data::DataFile>, app_config: &AppConfig, content: String) -> Result<()> {
    let id = d.sorted_ids().last().unwrap_or(&0) + 1;
    d.add(id, &content)?;
    let lines = d.as_string()?;
    data::write_file(&app_config.data_file_path(), &lines)?;
    Ok(())
}

pub fn remove(d: &mut Box<dyn data::DataFile>, app_config: &AppConfig, id: u32) -> Result<()> {
    d.remove(id)?;
    let lines = d.as_string()?;
    data::write_file(&app_config.data_file_path(), &lines)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::MemoData;

    #[test]
    fn test_app_config_new() {
        let app_config = AppConfig::new("memo", "memo.txt");
        let mut data_dir = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
        data_dir.push("memo");

        assert_eq!(app_config.name(), "memo");
        assert_eq!(app_config.data_file(), "memo.txt");
        assert_eq!(app_config.data_dir(), &data_dir);
    }

    #[test]
    fn test_app_config_name() {
        let app_config = AppConfig::new("memo", "memo.txt");

        assert_eq!(app_config.name(), "memo");
    }

    #[test]
    fn test_app_config_data_file_path() {
        let app_config = AppConfig::new("memo", "memo.txt");
        let mut path = dirs::data_dir().unwrap();
        path.push("memo");
        path.push("memo.txt");

        assert_eq!(app_config.data_file_path(), path);
    }

    #[test]
    fn test_app_config_data_dir() {
        let app_config = AppConfig::new("memo", "memo.txt");
        let mut data_dir = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
        data_dir.push("memo");

        assert_eq!(app_config.data_dir(), &data_dir);
    }

    #[test]
    fn test_app_config_data_file() {
        let app_config = AppConfig::new("memo", "memo.txt");
        assert_eq!(app_config.data_file(), "memo.txt");
    }

    #[test]
    fn test_list() {
        let mut box_memo_data: Box<dyn data::DataFile> = Box::new(MemoData::new());
        assert_eq!(list(&mut box_memo_data).is_ok(), true);
    }

    #[test]
    fn test_list_empty() {
        let mut box_memo_data: Box<dyn data::DataFile> = Box::new(MemoData::new());
        assert_eq!(list(&mut box_memo_data).is_ok(), true);
    }

    #[test]
    fn test_init() {
        let mut app_config = AppConfig::new("memo", "memo.txt");
        let dir = tempfile::tempdir().unwrap();
        app_config.data_dir = dir.path().to_path_buf();

        assert_eq!(init(&app_config).is_ok(), true);
    }

    #[test]
    fn test_init_file_exists() {
        let mut app_config = AppConfig::new("memo", "memo.txt");
        let dir = tempfile::tempdir().unwrap();
        app_config.data_dir = dir.path().to_path_buf();

        std::fs::File::create(app_config.data_file_path()).unwrap();

        assert_eq!(
            init(&app_config).unwrap_err().to_string(),
            format!(
                "File '{}' already exists",
                app_config.data_file_path().display()
            )
        );
    }

    #[test]
    fn test_add() {
        let mut app_config = AppConfig::new("memo", "memo.txt");
        let dir = tempfile::tempdir().unwrap();
        app_config.data_dir = dir.path().to_path_buf();

        // Create file
        std::fs::File::create(&app_config.data_file_path()).unwrap();

        let mut box_memo_data: Box<dyn data::DataFile> = Box::new(MemoData::new());
        let content = "test".to_string();

        assert_eq!(add(&mut box_memo_data, &app_config, content).is_ok(), true);
    }

    #[test]
    fn test_remove() {
        let mut app_config = AppConfig::new("memo", "memo.txt");
        let dir = tempfile::tempdir().unwrap();
        app_config.data_dir = dir.path().to_path_buf();

        // Create file
        std::fs::File::create(&app_config.data_file_path()).unwrap();

        let mut d = MemoData::new();
        let content = "test".to_string();
        data::DataFile::add(&mut d, 1, &content).unwrap();

        let mut box_memo_data: Box<dyn data::DataFile> = Box::new(d);
        assert_eq!(remove(&mut box_memo_data, &app_config, 1).is_ok(), true);
    }
}
