//! Safe, intentionally conservative HTML token utilities (not a full parser).
use crate::dom::Document;
pub fn empty_document(url: Option<String>)->Document { Document{url, ..Default::default()} }
pub fn parse_document(_input:&str, url:Option<String>)->Document { empty_document(url) }
