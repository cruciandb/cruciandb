use std::{
    collections::{HashMap, hash_map},
    fmt::Debug,
    hash::Hash,
};

use super::Storage;

pub struct MemStorage<K: Clone + Eq + Hash, V: Clone + Debug> {
    data: hash_map::HashMap<K, V>,
}

impl<K: Clone + Eq + Hash, V: Clone + Debug> Default for MemStorage<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Clone + Eq + Hash, V: Clone + Debug> MemStorage<K, V> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl<K: Clone + Eq + Hash, V: Clone + Debug> Storage for MemStorage<K, V> {
    type Key = K;

    type Value = V;

    fn get(&self, key: &Self::Key) -> anyhow::Result<Option<Self::Value>> {
        Ok(self.data.get(key).cloned())
    }

    fn set(&mut self, key: &Self::Key, value: Self::Value) -> anyhow::Result<Option<Self::Value>> {
        Ok(self.data.insert(key.clone(), value))
    }

    fn delete(&mut self, key: &Self::Key) -> anyhow::Result<Option<Self::Value>> {
        Ok(self.data.remove(key))
    }
}

#[cfg(test)]
mod tests {
    use crate::storage::Storage;

    use super::MemStorage;

    #[test]
    pub fn test_string() {
        let mut storage: MemStorage<String, String> = MemStorage::new();
        let key = "key".to_string();
        let value = "value".to_string();
        let next_value = "next_value".to_string();

        let res_get = storage.get(&key);
        assert!(res_get.is_ok());
        assert_eq!(None, res_get.unwrap());

        let res_set = storage.set(&key, value.clone());
        assert!(res_set.is_ok());
        assert_eq!(None, res_set.unwrap());

        let res_get = storage.get(&key);
        assert!(res_get.is_ok());
        assert_eq!(Some(value.clone()), res_get.unwrap());

        let res_set = storage.set(&key, next_value.clone());
        assert!(res_set.is_ok());
        assert_eq!(Some(value.clone()), res_set.unwrap());

        let res_delete = storage.delete(&key);
        assert!(res_delete.is_ok());
        assert_eq!(Some(next_value.clone()), res_delete.unwrap());

        let res_delete = storage.delete(&key);
        assert!(res_delete.is_ok());
        assert_eq!(None, res_delete.unwrap());
    }

    #[test]
    pub fn test_i32() {
        let mut storage: MemStorage<i32, i32> = MemStorage::new();
        let key = 1;
        let value = 2;
        let next_value = 3;

        let res_get = storage.get(&key);
        assert!(res_get.is_ok());
        assert_eq!(None, res_get.unwrap());

        let res_set = storage.set(&key, value.clone());
        assert!(res_set.is_ok());
        assert_eq!(None, res_set.unwrap());

        let res_get = storage.get(&key);
        assert!(res_get.is_ok());
        assert_eq!(Some(value.clone()), res_get.unwrap());

        let res_set = storage.set(&key, next_value.clone());
        assert!(res_set.is_ok());
        assert_eq!(Some(value.clone()), res_set.unwrap());

        let res_delete = storage.delete(&key);
        assert!(res_delete.is_ok());
        assert_eq!(Some(next_value.clone()), res_delete.unwrap());

        let res_delete = storage.delete(&key);
        assert!(res_delete.is_ok());
        assert_eq!(None, res_delete.unwrap());
    }

    #[test]
    pub fn test_usize_byte_array() {
        let mut storage: MemStorage<usize, Vec<u8>> = MemStorage::new();
        let key = 1;
        let value = vec![1, 1, 1, 1, 1, 1, 1, 1];
        let next_value = vec![2, 3, 1, 1, 1, 1, 1, 1];

        let res_get = storage.get(&key);
        assert!(res_get.is_ok());
        assert_eq!(None, res_get.unwrap());

        let res_set = storage.set(&key, value.clone());
        assert!(res_set.is_ok());
        assert_eq!(None, res_set.unwrap());

        let res_get = storage.get(&key);
        assert!(res_get.is_ok());
        assert_eq!(Some(value.clone()), res_get.unwrap());

        let res_set = storage.set(&key, next_value.clone());
        assert!(res_set.is_ok());
        assert_eq!(Some(value.clone()), res_set.unwrap());

        let res_delete = storage.delete(&key);
        assert!(res_delete.is_ok());
        assert_eq!(Some(next_value.clone()), res_delete.unwrap());

        let res_delete = storage.delete(&key);
        assert!(res_delete.is_ok());
        assert_eq!(None, res_delete.unwrap());
    }

    
}
