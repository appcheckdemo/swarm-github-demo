//! Style resolution boundary and conservative defaults.
use crate::{css::Stylesheet,dom::Node};
#[derive(Clone,Debug,Default)] pub struct ComputedStyle { pub display:Display, pub color:(u8,u8,u8,u8), pub width:Option<f32>, pub height:Option<f32> }
#[derive(Clone,Copy,Debug,Default,PartialEq)] pub enum Display { #[default] Block, Inline, None }
pub fn resolve(_node:&Node,_sheet:&Stylesheet)->ComputedStyle { ComputedStyle::default() }
