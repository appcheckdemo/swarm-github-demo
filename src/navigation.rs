use url::Url;
pub fn resolve(base:&str,target:&str)->Result<String,String>{Url::parse(base).map_err(|e|e.to_string()).and_then(|u|u.join(target).map(|x|x.to_string()).map_err(|e|e.to_string()))}
