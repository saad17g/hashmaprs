// shard_manager.rs

use crate::shard::Shard;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct ShardManager {
    shards: Vec<Shard>,
}

impl ShardManager {
    pub fn new(shard_count: usize) -> Self {
        let mut shards = Vec::with_capacity(shard_count);
        for _ in 0..shard_count {
            shards.push(Shard::new());
        }

        ShardManager { shards }
    }

    fn hash<T: Hash + Sized>(t: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        t.hash(&mut hasher);
        hasher.finish()
    }

    pub fn get_shard(&mut self, key: &str) -> &mut Shard {
        let shard_index = (Self::hash(&key) as usize) % self.shards.len();
        &mut self.shards[shard_index]
    }

    pub fn get_shard_index(&self, key: &str) -> usize {
        let shard_index = (Self::hash(&key) as usize) % self.shards.len();
        shard_index
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let shard_index = self.get_shard_index(key);
        self.shards[shard_index].get(key)
    }

    pub fn set(&mut self, key: String, value: String) -> usize {
        let shard_index = (Self::hash(&key) as usize) % self.shards.len();
        let shard = self.get_shard(&key);
        shard.set(key, value);
        shard_index
    }

    pub fn delete(&mut self, key: &str) -> Option<String> {
        let shard = self.get_shard(key);
        shard.delete(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shard_manager_new() {
        let shard_count = 4;
        let shard_manager = ShardManager::new(shard_count);
        assert_eq!(shard_manager.shards.len(), shard_count);
    }

    #[test]
    fn test_shard_manager_set_and_get() {
        let mut shard_manager = ShardManager::new(4);
        shard_manager.set("key1".to_string(), "value1".to_string());
        assert_eq!(shard_manager.get("key1"), Some("value1".to_string()));
    }

    #[test]
    fn test_shard_manager_get_non_existent() {
        let shard_manager = ShardManager::new(4);
        assert_eq!(shard_manager.get("non_existent_key"), None);
    }

    #[test]
    fn test_shard_manager_delete() {
        let mut shard_manager = ShardManager::new(4);
        shard_manager.set("key1".to_string(), "value1".to_string());
        let deleted_value = shard_manager.delete("key1");
        assert_eq!(deleted_value, Some("value1".to_string()));
        assert_eq!(shard_manager.get("key1"), None);
    }

    #[test]
    fn test_shard_manager_delete_non_existent() {
        let mut shard_manager = ShardManager::new(4);
        let deleted_value = shard_manager.delete("non_existent_key");
        assert_eq!(deleted_value, None);
    }

    #[test]
    fn test_shard_manager_consistent_hashing() {
        let mut shard_manager = ShardManager::new(4);
        let key = "consistent_key";
        let value = "consistent_value";
        shard_manager.set(key.to_string(), value.to_string());

        let shard_index = (ShardManager::hash(&key) as usize) % shard_manager.shards.len();
        assert_eq!(
            shard_manager.shards[shard_index].get(key),
            Some(value.to_string())
        );
    }
}
