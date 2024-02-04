use std::collections::HashMap;

pub type Headers = HashMap<Vec<u8>, Vec<u8>>;

/*
#[derive(Debug)]
pub struct Headers(HashMap<Vec<u8>, Vec<u8>>);
impl Headers {
    pub fn new() -> Headers {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, key: Vec<u8>, value: Vec<u8>) -> Option<Vec<u8>> {
        self.0.insert(key, value)
    }
}
*/
