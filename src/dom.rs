//! Small, owned DOM tree used by layout and scripting.
use std::collections::BTreeMap;
use crate::model::NodeId;

#[derive(Clone, Debug, PartialEq)]
pub enum NodeKind { Element(String), Text(String), Comment(String), Doctype(String) }

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub id: NodeId,
    pub kind: NodeKind,
    pub attributes: BTreeMap<String, String>,
    pub children: Vec<Node>,
}

impl Node {
    pub fn element(name: impl Into<String>) -> Self { Self { id: NodeId(0), kind: NodeKind::Element(name.into().to_ascii_lowercase()), attributes: BTreeMap::new(), children: vec![] } }
    pub fn text(value: impl Into<String>) -> Self { Self { id: NodeId(0), kind: NodeKind::Text(value.into()), attributes: BTreeMap::new(), children: vec![] } }
    pub fn text_content(&self) -> String { match &self.kind { NodeKind::Text(s) => s.clone(), _ => self.children.iter().map(Node::text_content).collect() } }
    pub fn text(&self) -> String { self.text_content() }
    pub fn tag_name(&self) -> Option<&str> { match &self.kind { NodeKind::Element(s) => Some(s), _ => None } }
    pub fn attr(&self, name: &str) -> Option<&str> { self.attributes.get(&name.to_ascii_lowercase()).map(String::as_str) }
    pub fn set_attr(&mut self, name: impl Into<String>, value: impl Into<String>) { self.attributes.insert(name.into().to_ascii_lowercase(), value.into()); }
    pub fn set_text(&mut self, value: impl Into<String>) { self.kind = NodeKind::Text(value.into()); self.children.clear(); }
    pub fn append_child(&mut self, child: Node) { self.children.push(child); }
    pub fn remove_child(&mut self, index: usize) -> Option<Node> { (index < self.children.len()).then(|| self.children.remove(index)) }
    pub fn find_by_id(&self, id: &str) -> Option<&Node> { if self.attr("id") == Some(id) { return Some(self); } self.children.iter().find_map(|n| n.find_by_id(id)) }
    pub fn find_by_tag(&self, tag: &str) -> Vec<&Node> { let mut out=Vec::new(); let wanted=tag.to_ascii_lowercase(); if self.tag_name()==Some(wanted.as_str()) { out.push(self); } for n in &self.children { out.extend(n.find_by_tag(tag)); } out }
    pub fn has_class(&self, class: &str) -> bool { self.attr("class").map(|s| s.split_whitespace().any(|x|x==class)).unwrap_or(false) }
}
#[derive(Clone, Debug, PartialEq)] pub struct Document { pub root: Node }
impl Document { pub fn new() -> Self { Self { root: Node::element("document") } } pub fn text(&self)->String { self.root.text_content() } pub fn get_element_by_id(&self,id:&str)->Option<&Node>{self.root.find_by_id(id)} }
#[cfg(test)] mod tests { use super::*; #[test] fn mutation_and_queries(){let mut n=Node::element("div");n.set_attr("id","x");n.set_attr("class","a b");n.append_child(Node::text("hi"));assert!(n.has_class("b"));assert_eq!(n.text(),"hi");assert!(n.remove_child(0).is_some());} }
