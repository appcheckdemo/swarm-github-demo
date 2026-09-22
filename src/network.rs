//! Synchronous transport interface; default transport never performs I/O.
#[derive(Clone,Debug)] pub struct Request { pub url:String }
#[derive(Clone,Debug)] pub struct Response { pub status:u16,pub body:Vec<u8> }
pub trait Transport { fn fetch(&self,request:Request)->Result<Response,NetworkError>; }
#[derive(Clone,Debug)] pub enum NetworkError { Unsupported, InvalidUrl, Io(String) }
#[derive(Default)] pub struct NullTransport;
impl Transport for NullTransport { fn fetch(&self,_:Request)->Result<Response,NetworkError>{Err(NetworkError::Unsupported)} }
