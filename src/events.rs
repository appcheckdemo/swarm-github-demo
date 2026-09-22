//! Input events shared by platform backends.
#[derive(Clone,Debug)]pub enum Event{Quit,Resize(u32,u32),Key(char)}
