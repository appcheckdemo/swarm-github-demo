//! Native window/input boundary. The default backend is intentionally inert.
use crate::events::Event;
pub trait Platform { fn poll_event(&mut self)->Option<Event>; fn present(&mut self); }
#[derive(Default)] pub struct NullPlatform;
impl Platform for NullPlatform { fn poll_event(&mut self)->Option<Event>{None} fn present(&mut self){} }
