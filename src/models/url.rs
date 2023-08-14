#[derive(serde::Serialize)]
pub struct UrlRecord {
    id: String,
    target: String,
    expire_in: usize,
}

impl UrlRecord {
    pub fn new(id: String, target: String, expire_in: usize) -> Self {
        Self {
            id,
            target,
            expire_in,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_url_record() {
        let url_record = UrlRecord::new("id".to_string(), "target".to_string(), 0);
        assert_eq!(url_record.id, "id");
        assert_eq!(url_record.target, "target");
        assert_eq!(url_record.expire_in, 0);
    }
}
