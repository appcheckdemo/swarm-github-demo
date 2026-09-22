use crate::model::{Document,Size};
pub fn paint(doc:&Document,size:Size)->Vec<u32>{ let mut p=vec![0xffffffff;(size.width as usize).saturating_mul(size.height as usize)]; let s=crate::dom::body_text(doc); for (i,b) in s.bytes().enumerate().take(p.len()){p[i]=0xff000000|(b as u32)*0x010101;} p }
