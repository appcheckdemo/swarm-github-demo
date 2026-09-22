//! Shared geometry and identifiers used by every pipeline stage.
#[derive(Clone, Copy, Debug, Default, PartialEq)] pub struct Point { pub x:f32, pub y:f32 }
#[derive(Clone, Copy, Debug, Default, PartialEq)] pub struct Size { pub width:f32, pub height:f32 }
#[derive(Clone, Copy, Debug, Default, PartialEq)] pub struct Rect { pub origin:Point, pub size:Size }
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)] pub struct NodeId(pub u64);
#[derive(Clone, Copy, Debug, PartialEq, Eq)] pub struct Color(pub u8,pub u8,pub u8,pub u8);
impl Default for Color { fn default()->Self { Self(255,255,255,255) } }
