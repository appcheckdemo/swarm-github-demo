//! Stable shared DOM, geometry, and display-list types.
#[derive(Clone,Debug,PartialEq)] pub struct Document{pub root:Node}
#[derive(Clone,Debug,PartialEq)] pub struct Node{pub kind:NodeKind,pub children:Vec<Node>}
#[derive(Clone,Debug,PartialEq)] pub enum NodeKind{Element(String),Text(String)}
impl Node{pub fn text(&self)->String{match &self.kind{NodeKind::Text(s)=>s.clone(),_=>self.children.iter().map(Node::text).collect::<Vec<_>>().join("")}}}
#[derive(Clone,Copy,Debug,Default,PartialEq)] pub struct Rect{pub x:u32,pub y:u32,pub width:u32,pub height:u32}
#[derive(Clone,Debug,PartialEq)] pub struct DisplayItem{pub rect:Rect,pub color:[u8;4],pub text:Option<String>}
#[derive(Clone,Debug,Default,PartialEq)] pub struct DisplayList{pub items:Vec<DisplayItem>}
