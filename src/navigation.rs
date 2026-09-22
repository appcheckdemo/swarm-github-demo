//! URL and local navigation requests.
use url::Url;pub fn parse(s:&str)->Result<Url,url::ParseError>{Url::parse(s).or_else(|_|Url::from_file_path(s).map_err(|_|url::ParseError::EmptyHost))}
