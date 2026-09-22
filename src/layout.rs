use crate::model::{Document,Rect,Size};
pub fn layout(_doc:&Document, size:Size)->Vec<Rect>{vec![Rect{x:0,y:0,width:size.width,height:size.height}]}
