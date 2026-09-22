use std::collections::BTreeMap;
#[derive(Clone, Debug, Default)] pub struct Document { pub root: Node }
#[derive(Clone, Debug)] pub struct Node { pub kind: NodeKind, pub children: Vec<Node> }
#[derive(Clone, Debug)] pub enum NodeKind { Element { name: String, attrs: BTreeMap<String,String> }, Text(String) }
impl Default for Node { fn default()->Self { Self { kind: NodeKind::Element{name:"body".into(),attrs:BTreeMap::new()}, children:vec![] } } }
#[derive(Clone, Copy, Debug, Default)] pub struct Size { pub width: u32, pub height: u32 }
#[derive(Clone, Copy, Debug, Default)] pub struct Rect { pub x:u32,pub y:u32,pub width:u32,pub height:u32 }
#[derive(Clone, Debug)] pub struct StyleBox { pub rect: Rect, pub color: [u8;4] }
