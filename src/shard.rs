use std::collections::HashMap;

pub struct Shard {
    data: HashMap<String, String>,
}

impl Shard {
    pub fn new() -> Self {
        Shard {
            data: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }

    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn delete(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shard_new() {
        let shard = Shard::new();
        assert!(shard.data.is_empty());
    }

    #[test]
    fn test_shard_set_and_get() {
        let mut shard = Shard::new();
        shard.set("key1".to_string(), "value1".to_string());
        assert_eq!(shard.get("key1"), Some("value1".to_string()));
    }

    #[test]
    fn test_shard_get_non_existent() {
        let shard = Shard::new();
        assert_eq!(shard.get("non_existent_key"), None);
    }

    #[test]
    fn test_shard_delete() {
        let mut shard = Shard::new();
        shard.set("key1".to_string(), "value1".to_string());
        let deleted_value = shard.delete("key1");
        assert_eq!(deleted_value, Some("value1".to_string()));
        assert_eq!(shard.get("key1"), None);
    }

    #[test]
    fn test_shard_delete_non_existent() {
        let mut shard = Shard::new();
        let deleted_value = shard.delete("non_existent_key");
        assert_eq!(deleted_value, None);
    }
}
