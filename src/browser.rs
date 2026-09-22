use crate::{html,model::{Document,Size},network,paint};
pub struct Browser { pub document:Document, pub size:Size, pub scroll:i32 }
impl Browser { pub fn load(source:&str,size:Size)->Result<Self,String>{let data=if source.starts_with("http") {network::fetch(source)?} else {std::fs::read_to_string(source).map_err(|e|e.to_string())?}; Ok(Self{document:html::parse(&String::from_utf8_lossy(&data)),size,scroll:0})} pub fn framebuffer(&self)->Vec<u32>{paint::paint(&self.document,self.size)} }
