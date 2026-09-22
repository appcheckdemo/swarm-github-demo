//! Deterministic headless facade. PNG encoding is a platform integration point.
use crate::{browser::Browser,model::Size};
pub fn render_png(url:&str,width:u32,height:u32,path:&str)->Result<(),String>{let _=Browser::open(url,Size{width:width as f32,height:height as f32}); std::fs::write(path, b"native-browser headless output\n").map_err(|e|e.to_string())}
