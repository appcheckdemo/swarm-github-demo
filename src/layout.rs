//! Small, deterministic layout primitives.  This module deliberately knows nothing
//! about HTML; callers turn their document tree into `Node`s.
use std::cmp::{max, min};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Edge { pub top: f32, pub right: f32, pub bottom: f32, pub left: f32 }
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Size { pub width: f32, pub height: f32 }
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect { pub x: f32, pub y: f32, pub width: f32, pub height: f32 }
impl Rect { pub fn right(self)->f32 {self.x+self.width} pub fn bottom(self)->f32 {self.y+self.height} pub fn intersect(self,b:Rect)->Rect { let x=self.x.max(b.x); let y=self.y.max(b.y); Rect{x,y,width:(self.right().min(b.right())-x).max(0.),height:(self.bottom().min(b.bottom())-y).max(0.)} } }
#[derive(Clone, Copy, Debug, PartialEq)] pub enum Display { Block, Inline, Flex, Grid, None }
#[derive(Clone, Copy, Debug, Default, PartialEq)] pub struct BoxStyle { pub margin:Edge,pub padding:Edge,pub border:Edge,pub background:[u8;4],pub overflow:bool }
#[derive(Clone, Debug, PartialEq)] pub struct Node { pub display:Display, pub style:BoxStyle, pub size:Option<Size>, pub text:Option<String>, pub children:Vec<Node> }
#[derive(Clone, Debug, PartialEq)] pub struct LayoutBox { pub rect:Rect,pub content:Rect,pub children:Vec<LayoutBox>,pub text:Option<String>,pub background:[u8;4] }
pub fn layout(root:&Node, viewport:Size)->LayoutBox { flow(root, Rect{x:0.,y:0.,width:viewport.width,height:viewport.height}, viewport.width) }
fn flow(n:&Node, containing:Rect, width:f32)->LayoutBox { let s=n.size.unwrap_or(Size{width:width-(n.style.margin.left+n.style.margin.right),height:if n.text.is_some(){16.}else{0.}}); let x=containing.x+n.style.margin.left; let y=containing.y+n.style.margin.top; let content=Rect{x:x+n.style.border.left+n.style.padding.left,y:y+n.style.border.top+n.style.padding.top,width:s.width.max(0.),height:s.height.max(0.)}; let mut cy=content.y; let mut kids=Vec::new(); for c in &n.children { let w=if matches!(n.display,Display::Flex|Display::Grid){content.width/(n.children.len().max(1) as f32)}else{content.width}; let mut k=flow(c,Rect{x:content.x,y:cy,width:w,height:content.height},w); if matches!(n.display,Display::Flex|Display::Grid){k.rect.x=content.x+kids.len() as f32*w;k.content.x=k.rect.x;} cy=k.rect.bottom(); kids.push(k); } let h=if s.height>0.{s.height}else{(cy-content.y)+n.style.padding.top+n.style.padding.bottom+n.style.border.top+n.style.border.bottom}; LayoutBox{rect:Rect{x,y,width:s.width+ n.style.padding.left+n.style.padding.right+n.style.border.left+n.style.border.right,height:h},content,children:kids,text:n.text.clone(),background:n.style.background} }

#[cfg(test)] mod tests { use super::*; #[test] fn block_flow_and_box_model(){let n=Node{display:Display::Block,style:BoxStyle{padding:Edge{top:2.,right:3.,bottom:4.,left:5.},..Default::default()},size:Some(Size{width:20.,height:10.}),text:None,children:vec![]};let b=layout(&n,Size{width:100.,height:100.});assert_eq!(b.rect.width,28.);assert_eq!(b.content.x,5.);} #[test] fn responsive_width(){let n=Node{display:Display::Block,style:Default::default(),size:None,text:None,children:vec![Node{display:Display::Block,style:Default::default(),size:Some(Size{width:10.,height:4.}),text:None,children:vec![]}]};assert_eq!(layout(&n,Size{width:40.,height:20.}).children[0].rect.x,0.);} }
