use std::sync::{Arc, RwLock};
use rand::Rng;

#[derive(Debug)]
pub struct SkipListNode<K, V> {
    key: K,
    value: V,
    next: Vec<Option<Arc<RwLock<SkipListNode<K, V>>>>>,
}

#[derive(Debug)]
pub struct SkipList<K, V> {
    head: Arc<RwLock<SkipListNode<K, V>>>,
    max_level: usize,
    size: RwLock<usize>,
}

pub struct SkipListIterator<K, V> {
    current: Option<Arc<RwLock<SkipListNode<K, V>>>>,
}

impl<K: Ord + Clone + Default, V: Clone + Default> SkipList<K, V> {
    pub fn new(max_level: usize) -> Self {
        let head = Arc::new(RwLock::new(SkipListNode {
            key: K::default(),
            value: V::default(),
            next: vec![None; max_level],
        }));
        Self {
            head,
            max_level,
            size: RwLock::new(0),
        }
    }

    fn random_level(&self) -> usize {
        let mut lvl = 1;
        let mut rng = rand::rng();
        while lvl < self.max_level && rng.random_bool(0.5) {
            lvl += 1;
        }
        lvl
    }

    pub fn insert(&self, key: K, value: V) -> bool {
        let mut update: Vec<Arc<RwLock<SkipListNode<K, V>>>> = vec![self.head.clone(); self.max_level];
        let mut x = self.head.clone();
        // Find update path
        for i in (0..self.max_level).rev() {
            loop {
                let next_opt = x.read().unwrap().next[i].clone();
                match next_opt {
                    Some(ref next) => {
                        if next.read().unwrap().key < key {
                            x = next.clone();
                        } else {
                            break;
                        }
                    }
                    None => break,
                }
            }
            update[i] = x.clone();
        }
        // Check if key exists
        let next_opt = update[0].read().unwrap().next[0].clone();
        if let Some(ref next) = next_opt {
            if next.read().unwrap().key == key {
                next.write().unwrap().value = value;
                return false;
            }
        }
        // Insert new node
        let lvl = self.random_level();
        let new_node = Arc::new(RwLock::new(SkipListNode {
            key: key.clone(),
            value,
            next: vec![None; self.max_level],
        }));
        for i in 0..lvl {
            let mut update_node = update[i].write().unwrap();
            new_node.write().unwrap().next[i] = update_node.next[i].clone();
            update_node.next[i] = Some(new_node.clone());
        }
        let mut size = self.size.write().unwrap();
        *size += 1;
        true
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let mut x = self.head.clone();
        for i in (0..self.max_level).rev() {
            loop {
                let next_opt = x.read().unwrap().next[i].clone();
                match next_opt {
                    Some(ref next) => {
                        if next.read().unwrap().key < *key {
                            x = next.clone();
                        } else {
                            break;
                        }
                    }
                    None => break,
                }
            }
        }
        let next_opt = x.read().unwrap().next[0].clone();
        if let Some(ref next) = next_opt {
            if next.read().unwrap().key == *key {
                return Some(next.read().unwrap().value.clone());
            }
        }
        None
    }

    pub fn iter(&self) -> SkipListIterator<K, V> {
        let first = self.head.read().unwrap().next[0].clone();
        SkipListIterator { current: first }
    }

    pub fn estimated_size(&self) -> usize {
        *self.size.read().unwrap()
    }
}

impl<K: Clone, V: Clone> Iterator for SkipListIterator<K, V> {
    type Item = (K, V);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.current.take() {
            let node_guard = node.read().unwrap();
            let item = (node_guard.key.clone(), node_guard.value.clone());
            self.current = node_guard.next[0].clone();
            Some(item)
        } else {
            None
        }
    }
}
