use anyhow::Result;
use memo::app;
use memo::data;

pub fn add(
    d: &mut impl data::DataFile,
    app_config: &app::AppConfig,
    content: String,
) -> Result<()> {
    let id = d.sorted_ids().last().unwrap_or(&0) + 1;
    d.add(id, &content)?;
    // Get lines from format
    let lines = format!("{}", d);
    data::write_file(&app_config.data_file_path(), &lines)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut app_config = app::AppConfig::new("memo", "memo.txt");
        let dir = tempfile::tempdir().unwrap();
        app_config.data_dir = dir.path().to_path_buf();

        // Create file
        std::fs::File::create(&app_config.data_file_path()).unwrap();

        let mut memo_data = data::MemoData::new();
        let content = "test".to_string();

        assert_eq!(add(&mut memo_data, &app_config, content).is_ok(), true);
    }
}
