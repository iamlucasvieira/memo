use anyhow::{anyhow, Result};
use memo::app;
use memo::data;

/// Removes memos from data file.
pub fn remove(
    d: &mut impl data::DataFile,
    app_config: &app::AppConfig,
    id: Vec<u32>,
) -> Result<()> {
    let mut ids_not_found = String::new();
    for i in id {
        if let Err(e) = d.remove(i) {
            ids_not_found.push_str(&format!("{}: {}\n", i, e));
        }
    }

    if !ids_not_found.is_empty() {
        return Err(anyhow!("\n{}", ids_not_found));
    }

    let lines = format!("{}", d);
    data::write_file(&app_config.data_file_path(), &lines)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use memo::models;

    #[test]
    fn test_remove() {
        let mut app_config = app::AppConfig::new("memo", "memo.txt");
        let dir = tempfile::tempdir().unwrap();
        app_config.data_dir = dir.path().to_path_buf();

        // Create file
        std::fs::File::create(&app_config.data_file_path()).unwrap();

        let mut memo_data = models::MemoData::new();
        let content = "test".to_string();
        data::DataFile::add(&mut memo_data, 1, &content).unwrap();

        assert_eq!(remove(&mut memo_data, &app_config, vec![1]).is_ok(), true);
    }

    #[test]
    fn test_remove_multiple() {
        let mut app_config = app::AppConfig::new("memo", "memo.txt");
        let dir = tempfile::tempdir().unwrap();
        app_config.data_dir = dir.path().to_path_buf();

        // Create file
        std::fs::File::create(&app_config.data_file_path()).unwrap();

        let mut memo_data = models::MemoData::new();
        let content = "test".to_string();
        data::DataFile::add(&mut memo_data, 1, &content).unwrap();
        data::DataFile::add(&mut memo_data, 2, &content).unwrap();
        data::DataFile::add(&mut memo_data, 3, &content).unwrap();

        assert_eq!(
            remove(&mut memo_data, &app_config, vec![1, 2, 3]).is_ok(),
            true
        );
    }

    #[test]
    fn test_remove_invalid() {
        let mut app_config = app::AppConfig::new("memo", "memo.txt");
        let dir = tempfile::tempdir().unwrap();
        app_config.data_dir = dir.path().to_path_buf();

        // Create file
        std::fs::File::create(&app_config.data_file_path()).unwrap();

        let mut memo_data = models::MemoData::new();
        let content = "test".to_string();
        data::DataFile::add(&mut memo_data, 1, &content).unwrap();

        assert_eq!(remove(&mut memo_data, &app_config, vec![2]).is_err(), true);

        assert_eq!(data::DataFile::sorted_ids(&memo_data).len(), 1);
    }
}
