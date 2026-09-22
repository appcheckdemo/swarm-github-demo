//! Platform independent, deterministic browser event queue.
use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    Quit, Resize(u32,u32), Key(char),
    Click { target: String }, Keyboard { target: String, key: String },
    Input { target: String, value: String }, Change { target: String, value: String },
    Submit { target: String }, Timer(u64), Fetch { url: String, ok: bool },
}
impl Event { pub fn is_terminal(&self)->bool { matches!(self,Self::Quit) } }

#[derive(Clone, Debug, Default)]
pub struct EventQueue { events: VecDeque<Event> }
impl EventQueue {
    pub fn push(&mut self,e:Event){self.events.push_back(e)}
    pub fn pop(&mut self)->Option<Event>{self.events.pop_front()}
    pub fn len(&self)->usize{self.events.len()}
    pub fn is_empty(&self)->bool{self.events.is_empty()}
    pub fn drain(&mut self)->Vec<Event>{self.events.drain(..).collect()}
}

#[cfg(test)] mod tests { use super::*; #[test] fn fifo(){let mut q=EventQueue::default();q.push(Event::Timer(1));q.push(Event::Timer(2));assert_eq!(q.pop(),Some(Event::Timer(1)));} }
