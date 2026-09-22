//! Input events crossing the platform boundary.
#[derive(Clone,Debug)] pub enum Event { Resize{width:u32,height:u32}, Pointer{ x:f32,y:f32, pressed:bool }, Key{code:String, pressed:bool}, Close }
