//! Browser orchestration facade shared by desktop and headless execution.
use crate::{dom::Document,html,model::Size,paint::DisplayList};
pub struct Browser { pub document:Document, pub viewport:Size }
impl Browser { pub fn open(url:impl Into<String>,viewport:Size)->Self{let u=url.into();Self{document:html::empty_document(Some(u)),viewport}} pub fn display_list(&self)->DisplayList{DisplayList::default()} }
