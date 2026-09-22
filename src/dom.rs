use crate::model::{Document,Node,NodeKind};
pub fn text(node:&Node)->String { match &node.kind { NodeKind::Text(s)=>s.clone(), _=>node.children.iter().map(text).collect::<Vec<_>>().join("") } }
pub fn walk(node:&Node, out:&mut Vec<String>) { if let NodeKind::Element{name,..}=&node.kind { out.push(name.clone()) }; for c in &node.children { walk(c,out) } }
pub fn body_text(document:&Document)->String { text(&document.root) }
