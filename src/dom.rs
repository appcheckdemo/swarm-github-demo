//! Small owned DOM model; parsing is intentionally supplied by the caller.
use crate::model::NodeId;
#[derive(Clone, Debug, PartialEq)] pub enum NodeKind { Document, Element { tag:String }, Text(String), Comment(String) }
#[derive(Clone, Debug)] pub struct Node { pub id:NodeId, pub kind:NodeKind, pub children:Vec<Node>, pub attributes:Vec<(String,String)> }
impl Node { pub fn document()->Self{Self{id:NodeId(0),kind:NodeKind::Document,children:vec![],attributes:vec![]}} pub fn element(id:NodeId,tag:impl Into<String>)->Self{Self{id,kind:NodeKind::Element{tag:tag.into()},children:vec![],attributes:vec![]}} pub fn append(&mut self,n:Node){self.children.push(n)} }
#[derive(Clone, Debug)] pub struct Document { pub root:Node, pub url:Option<String> }
impl Default for Document { fn default()->Self{Self{root:Node::document(),url:None}} }
