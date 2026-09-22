//! Bounded resource resolution and tiny image decoder helpers.
use std::collections::HashMap;
#[derive(Default)] pub struct Resources { pub cache:HashMap<String,Option<Vec<u8>>>, pub max_bytes:usize }
impl Resources { pub fn new()->Self{Self{cache:HashMap::new(),max_bytes:8*1024*1024}} pub fn resolve(&mut self,url:&str)->Option<Vec<u8>>{if let Some(v)=self.cache.get(url){return v.clone()} let v=if url.starts_with("data:"){url.split_once(',').and_then(|(_,x)|{if x.len()<=self.max_bytes{Some(x.as_bytes().to_vec())}else{None}})}else{None};self.cache.insert(url.to_string(),v.clone());v} pub fn font(&self,requested:&str)->String{if requested.trim().is_empty(){"sans-serif".into()}else{requested.into()}} }
#[cfg(test)] mod tests{use super::*;#[test]fn cache_and_bound(){let mut r=Resources::new();assert_eq!(r.resolve("data:text/plain,ok"),Some(b"ok".to_vec()));assert_eq!(r.resolve("missing"),None);}}
