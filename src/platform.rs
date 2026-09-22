//! Native window abstraction.
pub trait Window{fn present(&mut self,pixels:&[u32],width:u32,height:u32);}
