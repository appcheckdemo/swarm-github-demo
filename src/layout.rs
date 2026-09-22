//! Layout produces geometry without painting or platform dependencies.
use crate::{dom::Node,model::{Rect,Size},style::ComputedStyle};
#[derive(Clone,Debug)] pub struct LayoutBox { pub rect:Rect, pub node:Option<crate::model::NodeId>, pub children:Vec<LayoutBox> }
pub fn layout(node:&Node, _style:&ComputedStyle, viewport:Size)->LayoutBox { LayoutBox{rect:Rect{origin:Default::default(),size:viewport},node:Some(node.id),children:vec![]} }
