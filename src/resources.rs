pub fn load(path:&str)->Result<Vec<u8>,String>{std::fs::read(path).map_err(|e|format!("cannot read {path}: {e}"))}
