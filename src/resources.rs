//! Resource cache and bytes abstraction; network policy lives in `network`.
use std::collections::HashMap;
#[derive(Default)] pub struct ResourceStore { pub bytes:HashMap<String,Vec<u8>> }
impl ResourceStore { pub fn insert(&mut self,url:impl Into<String>,data:Vec<u8>){self.bytes.insert(url.into(),data);} pub fn get(&self,url:&str)->Option<&[u8]>{self.bytes.get(url).map(Vec::as_slice)} }
