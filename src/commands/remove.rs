use anyhow::Result;
use memo::app;
use memo::data;

pub fn remove(d: &mut impl data::DataFile, app_config: &app::AppConfig, id: u32) -> Result<()> {
    d.remove(id)?;
    let lines = format!("{}", d);
    data::write_file(&app_config.data_file_path(), &lines)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove() {
        let mut app_config = app::AppConfig::new("memo", "memo.txt");
        let dir = tempfile::tempdir().unwrap();
        app_config.data_dir = dir.path().to_path_buf();

        // Create file
        std::fs::File::create(&app_config.data_file_path()).unwrap();

        let mut memo_data = data::MemoData::new();
        let content = "test".to_string();
        data::DataFile::add(&mut memo_data, 1, &content).unwrap();

        assert_eq!(remove(&mut memo_data, &app_config, 1).is_ok(), true);
    }
}
