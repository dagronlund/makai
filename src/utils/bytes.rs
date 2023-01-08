use std::collections::HashMap;

use bytes::Bytes;

#[derive(Clone, Debug)]
pub struct ByteStorage {
    map: HashMap<Bytes, usize>,
    vec: Vec<Bytes>,
}

impl ByteStorage {
    pub fn new() -> Self {
        let mut storage = Self {
            map: HashMap::new(),
            vec: Vec::new(),
        };
        // Initialize index 0 to the empty string
        storage.insert(Bytes::from_static(b""));
        storage
    }

    pub fn insert(&mut self, bytes: Bytes) -> usize {
        if let Some(name_id) = self.map.get(&bytes) {
            *name_id
        } else {
            let name_id = self.vec.len();
            self.vec.push(bytes.clone());
            self.map.insert(bytes, name_id);
            name_id
        }
    }

    pub fn get_bytes(&self, id: usize) -> Bytes {
        self.vec[id].clone()
    }
}

impl Default for ByteStorage {
    fn default() -> Self {
        Self::new()
    }
}
