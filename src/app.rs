use std::path::PathBuf;

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
