//! Platform-neutral display list, suitable for a native raster backend.
use crate::model::{Color,Rect};
#[derive(Clone,Debug)] pub enum DisplayCommand { FillRect{rect:Rect,color:Color}, Text{at:crate::model::Point,text:String,color:Color} }
#[derive(Clone,Debug,Default)] pub struct DisplayList { pub commands:Vec<DisplayCommand> }
