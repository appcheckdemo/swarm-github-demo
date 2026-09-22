//! Input events shared by platform backends.

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    Quit,
    Resize(u32, u32),
    Key(char),
}

impl Event {
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Quit)
    }
}

#[derive(Clone, Debug, Default)]
pub struct EventQueue {
    events: std::collections::VecDeque<Event>,
}

impl EventQueue {
    pub fn push(&mut self, event: Event) {
        self.events.push_back(event);
    }

    pub fn pop(&mut self) -> Option<Event> {
        self.events.pop_front()
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}
