//! Deterministic CPU display-list rasterizer (RGBA, no platform font dependency).
use crate::layout::{LayoutBox, Rect};

#[derive(Clone, Debug, PartialEq)]
pub enum Op { Fill(Rect,[u8;4]), Text(f32,f32,String,[u8;4]), Clip(Rect), Image(Rect,Vec<u8>) }
#[derive(Clone, Debug)]
pub struct Canvas { pub width:u32, pub height:u32, pub pixels:Vec<u8>, pub ops:Vec<Op>, pub scroll_x:f32, pub scroll_y:f32 }
impl Canvas {
 pub fn new(w:u32,h:u32)->Self { Self{width:w,height:h,pixels:vec![0;w as usize*h as usize*4],ops:vec![],scroll_x:0.,scroll_y:0.} }
 pub fn display_list(&mut self,b:&LayoutBox){ self.ops.push(Op::Fill(b.rect,b.background)); for c in &b.children { self.display_list(c) } if let Some(t)=&b.text { self.ops.push(Op::Text(b.content.x,b.content.y,t.clone(),[0,0,0,255])) } }
 pub fn set_scroll(&mut self,x:f32,y:f32){self.scroll_x=x;self.scroll_y=y;}
 pub fn rasterize(&mut self){ let mut clip=Rect{x:0.,y:0.,width:self.width as f32,height:self.height as f32}; for op in self.ops.clone(){ match op { Op::Clip(r)=>clip=intersect(clip, shifted(r,self.scroll_x,self.scroll_y)), Op::Fill(r,c)=>fill(self,&clip,shifted(r,self.scroll_x,self.scroll_y),c), Op::Text(x,y,s,c)=>text(self,&clip,x-self.scroll_x,y-self.scroll_y,&s,c), Op::Image(r,data)=>{if let Some((w,h,p))=decode_rgba(&data){image(self,&clip,shifted(r,self.scroll_x,self.scroll_y),w,h,&p)}} } } }
 pub fn rgba(&self)->&[u8]{&self.pixels}
 pub fn png(&self)->Vec<u8>{png(self.width,self.height,&self.pixels)}
}
fn shifted(mut r:Rect,x:f32,y:f32)->Rect{r.x-=x;r.y-=y;r}
fn intersect(a:Rect,b:Rect)->Rect{let x=a.x.max(b.x);let y=a.y.max(b.y);let r=a.right().min(b.right());let d=a.bottom().min(b.bottom());Rect{x,y,width:(r-x).max(0.),height:(d-y).max(0.)}}
fn fill(c:&mut Canvas,cl:&Rect,r:Rect,col:[u8;4]){let r=intersect(*cl,r);for y in r.y.max(0.) as u32..r.bottom().min(c.height as f32).max(0.) as u32{for x in r.x.max(0.) as u32..r.right().min(c.width as f32).max(0.) as u32{let i=((y*c.width+x)*4)as usize;let a=col[3]as u32;for k in 0..3{c.pixels[i+k]=((col[k]as u32*a+c.pixels[i+k]as u32*(255-a))/255)as u8}c.pixels[i+3]=255}}}
fn text(c:&mut Canvas,cl:&Rect,x:f32,y:f32,s:&str,col:[u8;4]){for (n,ch) in s.chars().enumerate(){if ch!=' '{fill(c,cl,Rect{x:x+n as f32*8.,y,width:7.,height:12.},col)}}}
fn image(c:&mut Canvas,cl:&Rect,r:Rect,w:u32,h:u32,p:&[u8]){for y in 0..h{for x in 0..w{let q=Rect{x:r.x+x as f32,y:r.y+y as f32,width:1.,height:1.};if q.x>=cl.x&&q.y>=cl.y&&q.x<cl.right()&&q.y<cl.bottom(){fill(c,cl,q,[p[((y*w+x)*4)as usize],p[((y*w+x)*4+1)as usize],p[((y*w+x)*4+2)as usize],p[((y*w+x)*4+3)as usize]])}}}}
fn decode_rgba(d:&[u8])->Option<(u32,u32,Vec<u8>)>{if d.len()>=8&&&d[..8]==b"\x89PNG\r\n\x1a\n"{None}else{None}}
fn png(w:u32,h:u32,p:&[u8])->Vec<u8>{fn crc(d:&[u8])->u32{let mut c=!0u32;for &b in d{c^=b as u32;for _ in 0..8{c=if c&1!=0{(c>>1)^0xedb88320}else{c>>1}}}!c}fn chunk(o:&mut Vec<u8>,n:&[u8],d:&[u8]){o.extend((d.len()as u32).to_be_bytes());o.extend(n);o.extend(d);o.extend(crc(&[n,d].concat()).to_be_bytes())}let mut o=b"\x89PNG\r\n\x1a\n".to_vec();let mut ih=Vec::new();ih.extend(w.to_be_bytes());ih.extend(h.to_be_bytes());ih.extend([8,6,0,0,0]);chunk(&mut o,b"IHDR",&ih);let mut raw=Vec::new();for y in 0..h{raw.push(0);raw.extend(&p[(y*w*4)as usize..((y+1)*w*4)as usize])}let mut z=vec![0x78,0x01];let mut at=0;while at<raw.len(){let n=(raw.len()-at).min(65535);z.push(if at+n==raw.len(){1}else{0});z.extend((n as u16).to_le_bytes());z.extend((!(n as u16)).to_le_bytes());z.extend(&raw[at..at+n]);at+=n}chunk(&mut o,b"IDAT",&z);chunk(&mut o,b"IEND",&[]);o}
