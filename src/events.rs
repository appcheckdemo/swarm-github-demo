#[derive(Clone,Debug)] pub enum Event { Click{ x:u32,y:u32 }, Scroll{ delta:i32 }, Resize{width:u32,height:u32} }
