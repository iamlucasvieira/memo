use anyhow::Result;
use memo::data;

pub fn list(d: &impl data::DataFile) -> Result<()> {
    println!("{}", d);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use memo::models;

    #[test]
    fn test_list() {
        let memo_data = models::MemoData::new();
        assert_eq!(list(&memo_data).is_ok(), true);
    }

    #[test]
    fn test_list_empty() {
        let memo_data = models::MemoData::new();
        assert_eq!(list(&memo_data).is_ok(), true);
    }
}
