use anyhow::{anyhow, Context, Result};
use memo::app;

pub fn init(app_config: &app::AppConfig) -> Result<()> {
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
    std::fs::File::create(app_config.data_file_path()).with_context(|| {
        format!(
            "Could not create file '{}'",
            app_config.data_file_path().display()
        )
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_init() {
        let mut app_config = app::AppConfig::new("memo", "memo.txt");
        let dir = tempfile::tempdir().unwrap();
        app_config.data_dir = dir.path().to_path_buf();

        assert_eq!(init(&app_config).is_ok(), true);
    }

    #[test]
    fn test_init_file_exists() {
        let mut app_config = app::AppConfig::new("memo", "memo.txt");
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
}
