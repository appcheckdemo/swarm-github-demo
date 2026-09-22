#[derive(Clone,Debug,Default)] pub struct Stylesheet { pub background:[u8;4], pub text:[u8;4] }
pub fn parse(s:&str)->Stylesheet { let mut x=Stylesheet{background:[255,255,255,255],text:[20,20,20,255]}; if s.contains("background:black"){x.background=[0,0,0,255]} x }
