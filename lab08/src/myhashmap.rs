use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::Map61B;

const STARTING_BUCKETS: usize = 10;
const RESIZE_FACTOR: usize = 4; // newlen = r * (# elements)
const MAX_LOAD_FACTOR: f64 = 0.75;
const MIN_LOAD_FACTOR: f64 = 0.05; // only applicable when buckets.len() > STARTING_BUCKETS

pub struct MyHashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    num_elements: usize,
}

/*
The following helper functions may be useful. Feel free to delete or modify them.
*/

fn hash_index<K: Hash>(key: &K, len: usize) -> usize {
    let mut h = DefaultHasher::new();
    key.hash(&mut h);
    (h.finish() % (len as u64)) as usize
}

fn create_buckets<T>(num_buckets: usize) -> Vec<Vec<T>> {
    (0..num_buckets).map(|_| Vec::new()).collect()
}

fn loading<K, V>(h: &MyHashMap<K, V>) -> f64 {
    h.num_elements as f64 / h.buckets.len() as f64
}

impl<K: Hash, V> MyHashMap<K, V> {
    fn resize(&mut self, capacity: usize) {
        let mut new_buckets = create_buckets(capacity);
        let cur_buckets = std::mem::take(&mut self.buckets);
        for bucket in cur_buckets.into_iter() {
            for item in bucket.into_iter() {
                let index = hash_index(&item.0, new_buckets.len());
                new_buckets[index].push((item.0, item.1));
            }
        }
        self.buckets = new_buckets;
    }
}

impl<K, V> IntoIterator for MyHashMap<K, V> {
    type Item = (K, V);
    type IntoIter = std::vec::IntoIter<(K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        unimplemented!()
    }
}

impl<K: Hash + Eq, V> Map61B for MyHashMap<K, V> {
    type Key = K;

    type Value = V;

    fn new() -> Self {
        MyHashMap {
            buckets: create_buckets(STARTING_BUCKETS),
            num_elements: 0,
        }
    }

    fn len(&self) -> usize {
        self.num_elements
    }

    fn clear(&mut self) {
        self.buckets.clear();
        self.buckets = create_buckets(STARTING_BUCKETS);
        self.num_elements = 0;
    }

    fn contains_key(&self, key: &Self::Key) -> bool {
        self.get(key).is_some()
    }

    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        if loading(self) > MAX_LOAD_FACTOR {
            self.resize(self.num_elements * RESIZE_FACTOR);
        }
        let index = hash_index(&key, self.buckets.len());
        let bucket = self.buckets.get_mut(index).unwrap();
        for item in bucket.iter_mut() {
            if key == item.0 {
                let prev = std::mem::replace(&mut item.1, value);
                return Some(prev);
            }
        }
        bucket.push((key, value));
        self.num_elements += 1;
        None
    }

    fn get(&self, key: &Self::Key) -> Option<&Self::Value> {
        let index = hash_index(&key, self.buckets.len());
        let bucket = self.buckets.get(index);
        match bucket {
            Some(b) => {
                for item in b.iter() {
                    if *key == item.0 {
                        return Some(&item.1);
                    }
                }
                None
            }
            None => None,
        }
    }

    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value> {
        let index = hash_index(&key, self.buckets.len());
        let bucket = self.buckets.get_mut(index);
        match bucket {
            Some(b) => {
                for item in b.iter_mut() {
                    if *key == item.0 {
                        return Some(&mut item.1);
                    }
                }
                None
            }
            None => None,
        }
    }

    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value> {
        unimplemented!()
    }
}
