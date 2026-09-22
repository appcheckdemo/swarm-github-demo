//! Network extension point; fetching is explicit.
pub fn fetch(_url:&str)->Result<Vec<u8>,String>{Err("network backend not configured".into())}
