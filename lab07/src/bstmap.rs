use crate::Map61B;

struct BSTNode<K, V> {
    key: K,
    value: V,
    left: Option<Box<BSTNode<K, V>>>,
    right: Option<Box<BSTNode<K, V>>>,
}

pub struct BSTMap<K, V> {
    root: Option<Box<BSTNode<K, V>>>,
    size: usize,
}

impl<K, V> IntoIterator for BSTMap<K, V> {
    type Item = (K, V);
    type IntoIter = std::vec::IntoIter<(K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        unimplemented!()
    }
}

impl<K: Ord, V> Map61B for BSTMap<K, V> {
    type Key = K;
    type Value = V;

    fn new() -> Self {
        BSTMap {
            root: None,
            size: 0,
        }
    }

    fn len(&self) -> usize {
        self.size
    }

    fn clear(&mut self) {
        self.size = 0;
        self.root = None;
    }

    fn contains_key(&self, key: &Self::Key) -> bool {
        self.get(key).is_some()
    }

    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        let mut node = &mut self.root;

        while let Some(n) = node {
            match key.cmp(&n.key) {
                std::cmp::Ordering::Less => node = &mut n.left,
                std::cmp::Ordering::Equal => {
                    let prev = std::mem::replace(&mut n.value, value);
                    return Some(prev);
                }
                std::cmp::Ordering::Greater => node = &mut n.right,
            }
        }
        *node = Some(Box::from(BSTNode {
            key,
            value,
            left: None,
            right: None,
        }));

        self.size += 1;
        None
    }

    fn get(&self, key: &Self::Key) -> Option<&Self::Value> {
        let mut node = &self.root;

        while let Some(n) = node {
            match key.cmp(&n.key) {
                std::cmp::Ordering::Less => node = &n.left,
                std::cmp::Ordering::Equal => return Some(&n.value),
                std::cmp::Ordering::Greater => node = &n.right,
            }
        }
        None
    }

    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value> {
        let mut node = &mut self.root;

        while let Some(n) = node {
            match key.cmp(&n.key) {
                std::cmp::Ordering::Less => node = &mut n.left,
                std::cmp::Ordering::Equal => return Some(&mut n.value),
                std::cmp::Ordering::Greater => node = &mut n.right,
            }
        }
        None
    }

    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value> {
        unimplemented!()
    }
}
