//! URL navigation state machine independent of a window.
#[derive(Default)] pub struct Navigation { pub current:Option<String>, pub history:Vec<String> }
impl Navigation { pub fn navigate(&mut self,url:impl Into<String>){let u=url.into();self.current=Some(u.clone());self.history.push(u);} }
