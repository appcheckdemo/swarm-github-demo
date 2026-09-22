//! Bounded resource resolution and deterministic data URL helpers.
use std::collections::HashMap;
#[derive(Default)] pub struct Resources { pub cache:HashMap<String,Option<Vec<u8>>>, pub max_bytes:usize }
impl Resources {
 pub fn new()->Self{Self{cache:HashMap::new(),max_bytes:8*1024*1024}}
 pub fn resolve(&mut self,url:&str)->Option<Vec<u8>>{if let Some(v)=self.cache.get(url){return v.clone()}let v=if let Some((head,body))=url.split_once(','){if !head.starts_with("data:"){None}else if body.len()>self.max_bytes*2{None}else if head.to_ascii_lowercase().contains(";base64"){decode64(body).filter(|x|x.len()<=self.max_bytes)}else{Some(percent(body).filter(|x|x.len()<=self.max_bytes)?)} }else{None};self.cache.insert(url.to_string(),v.clone());v}
 pub fn font(&self,requested:&str)->String{let s=requested.trim();if s.is_empty(){"sans-serif".into()}else{s.chars().filter(|c|c.is_ascii_graphic()||c.is_ascii_whitespace()).collect()}}
}
fn percent(s:&str)->Option<Vec<u8>>{let b=s.as_bytes();let mut o=Vec::new();let mut i=0;while i<b.len(){if b[i]==b'%'{if i+2>=b.len(){return None}o.push((hex(b[i+1])?*16+hex(b[i+2])?)as u8);i+=3}else{o.push(b[i]);i+=1}}Some(o)}fn hex(c:u8)->Option<u8>{match c{b'0'..=b'9'=>Some(c-b'0'),b'a'..=b'f'=>Some(c-b'a'+10),b'A'..=b'F'=>Some(c-b'A'+10),_= >None}}
fn decode64(s:&str)->Option<Vec<u8>>{let mut o=Vec::new();let mut v=0u32;let mut n=0;for c in s.bytes().filter(|c|!c.is_ascii_whitespace()){if c==b'='{break}let x=match c{b'A'..=b'Z'=>c-b'A',b'a'..=b'z'=>c-b'a'+26,b'0'..=b'9'=>c-b'0'+52,b'+'=>62,b'/'=>63,_=>return None};v=(v<<6)|x as u32;n+=6;if n>=8{n-=8;o.push((v>>n)as u8)}}Some(o)}
