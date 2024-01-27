use anyhow::Result;
use memo::data;

pub fn list(d: &impl data::DataFile, mode: data::DisplayMode) -> Result<()> {
    d.display(mode)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use memo::models;

    #[test]
    fn test_list() {
        let memo_data = models::MemoData::new();
        assert_eq!(
            list(&memo_data, data::DisplayMode::GroupByDate).is_ok(),
            true
        );
        assert_eq!(list(&memo_data, data::DisplayMode::Sorted).is_ok(), true);
    }

    #[test]
    fn test_list_empty() {
        let memo_data = models::MemoData::new();
        assert_eq!(
            list(&memo_data, data::DisplayMode::GroupByDate).is_ok(),
            true
        );
        assert_eq!(list(&memo_data, data::DisplayMode::Sorted).is_ok(), true);
    }
}
