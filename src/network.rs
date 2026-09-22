//! Small, synchronous network facade.  Policy (timeouts and TLS validation) is
//! deliberately explicit so callers cannot accidentally turn validation off.
use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

#[derive(Clone, Debug)] pub struct Request { pub url:String, pub method:String, pub body:Option<Vec<u8>> }
impl Request { pub fn get(url: impl Into<String>) -> Self { Self{url:url.into(),method:"GET".into(),body:None} } }
#[derive(Clone, Debug)] pub struct Response { pub status:u16,pub body:Vec<u8>,pub headers:Vec<(String,String)> }
#[derive(Clone, Debug, PartialEq, Eq)] pub enum NetworkError { InvalidUrl, UnsupportedScheme, Io(String), InvalidTls, Cancelled, Http(u16) }
pub trait Transport { fn fetch(&self, request:Request)->Result<Response,NetworkError>; }
#[derive(Clone,Debug)] pub struct HttpTransport { pub timeout:Duration, pub max_redirects:usize }
impl Default for HttpTransport { fn default()->Self{Self{timeout:Duration::from_secs(15),max_redirects:10}} }
impl Transport for HttpTransport {
 fn fetch(&self,r:Request)->Result<Response,NetworkError>{
  let u=r.url.trim(); if u.starts_with("file://") { return fs::read(&u[7..]).map(|body|Response{status:200,body,headers:vec![]}).map_err(|e|NetworkError::Io(e.to_string())); }
  let rest=u.strip_prefix("http://").ok_or_else(||if u.starts_with("https://"){NetworkError::InvalidTls}else{NetworkError::UnsupportedScheme})?;
  let (host,path)=rest.split_once('/').unwrap_or((rest,"")); if host.is_empty(){return Err(NetworkError::InvalidUrl)}
  let mut s=TcpStream::connect((host.split(':').next().unwrap(),host.split(':').nth(1).and_then(|x|x.parse().ok()).unwrap_or(80))).map_err(|e|NetworkError::Io(e.to_string()))?;
  s.set_read_timeout(Some(self.timeout)).ok(); s.set_write_timeout(Some(self.timeout)).ok();
  let body=r.body.unwrap_or_default(); let head=format!("{} /{} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\nContent-Length: {}\r\n\r\n",r.method,path,host,body.len()); s.write_all(head.as_bytes()).and_then(|_|s.write_all(&body)).map_err(|e|NetworkError::Io(e.to_string()))?;
  let mut out=Vec::new(); s.read_to_end(&mut out).map_err(|e|NetworkError::Io(e.to_string()))?; let split=out.windows(4).position(|w|w==b"\r\n\r\n").ok_or(NetworkError::InvalidUrl)?; let h=String::from_utf8_lossy(&out[..split]); let status=h.lines().next().and_then(|x|x.split_whitespace().nth(1)).and_then(|x|x.parse().ok()).ok_or(NetworkError::InvalidUrl)?; Ok(Response{status,body:out[split+4..].to_vec(),headers:vec![]})
 }
}
pub fn resolve(base:&str, target:&str)->String { if target.contains("://")||target.starts_with("file:"){return target.into()} let (prefix,_)=base.rsplit_once('/').unwrap_or((base,"")); if target.starts_with('/') { let scheme=base.split_once("//").map(|x|x.0).unwrap_or(""); return format!("{}//{}",scheme,target) } format!("{}/{}",prefix,target) }
#[derive(Default)] pub struct NullTransport;
impl Transport for NullTransport { fn fetch(&self,_:Request)->Result<Response,NetworkError>{Err(NetworkError::UnsupportedScheme)} }
