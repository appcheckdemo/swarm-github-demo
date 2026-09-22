//! Browser state coordinating DOM and display list.
use crate::{html,layout};use crate::model::*;pub struct Browser{pub document:Document,pub display_list:DisplayList,pub width:u32}impl Browser{pub fn from_html(s:&str,w:u32)->Self{let document=html::parse(s);let display_list=layout::layout(&document,w);Self{document,display_list,width:w}}}
