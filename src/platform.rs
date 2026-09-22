//! Small native desktop surface.  This deliberately uses pixels rather than a
//! WebView: the browser's paint buffer is copied into the viewport.
use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Window as MfWindow, WindowOptions};

pub trait Window { fn present(&mut self, pixels: &[u32], width: u32, height: u32); }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Control { Back, Forward, Reload, Stop, Address }

pub fn hit_test(x: usize, y: usize, width: usize) -> Option<Control> {
    if y >= 36 { return None }
    if x < 36 { Some(Control::Back) }
    else if x < 72 { Some(Control::Forward) }
    else if x < 108 { Some(Control::Reload) }
    else if x < 144 { Some(Control::Stop) }
    else if x < width { Some(Control::Address) } else { None }
}

#[derive(Clone, Debug, Default)]
pub struct AddressBar { pub text: String, pub cursor: usize }
impl AddressBar {
    pub fn new(text: impl Into<String>) -> Self { let text=text.into(); let cursor=text.len(); Self{text,cursor} }
    pub fn insert(&mut self, c: char) { self.text.insert(self.cursor,c); self.cursor+=c.len_utf8(); }
    pub fn backspace(&mut self) { if self.cursor>0 { let n=self.text[..self.cursor].char_indices().last().map(|(i,_)|i).unwrap_or(0); self.text.drain(n..self.cursor); self.cursor=n; } }
    pub fn move_left(&mut self) { self.cursor=self.text[..self.cursor].char_indices().last().map(|(i,_)|i).unwrap_or(0); }
    pub fn move_right(&mut self) { self.cursor+=self.text[self.cursor..].chars().next().map(char::len_utf8).unwrap_or(0); }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Input { Back, Forward, Reload, Stop, Navigate, Key(Key), Mouse { x: usize, y: usize, down: bool } }

/// A native, pixel-backed browser window. `poll` returns user actions while
/// `present` paints the browser image below the simple native chrome.
pub struct DesktopWindow { window: MfWindow, buffer: Vec<u32>, width: usize, height: usize,
    pub address: AddressBar, pub loading: bool, pub error: Option<String> }
impl DesktopWindow {
    pub fn new(title: &str, width: usize, height: usize) -> Result<Self,String> {
        if width == 0 || height < 37 { return Err("display surface has invalid dimensions".into()) }
        let window=MfWindow::new(title,width,height,WindowOptions::default()).map_err(|e| format!("cannot open desktop display: {e}"))?;
        Ok(Self{window,buffer:vec![0xff202124;width*height],width,height,address:AddressBar::new("about:blank"),loading:false,error:None})
    }
    pub fn is_open(&self)->bool { self.window.is_open() }
    pub fn poll(&mut self)->Vec<Input> {
        let mut out=Vec::new();
        if let Some((x,y))=self.window.get_mouse_pos(MouseMode::Discard) { if self.window.get_mouse_down(MouseButton::Left) { out.push(Input::Mouse{x:x as usize,y:y as usize,down:true}); } }
        for k in self.window.get_keys_pressed(KeyRepeat::Yes) { out.push(Input::Key(k)); }
        if self.window.is_key_pressed(Key::Escape, KeyRepeat::No) { out.push(Input::Stop); }
        out
    }
    pub fn navigate(&mut self)->Input { self.loading=true; Input::Navigate }
    pub fn handle_control(&mut self,x:usize,y:usize)->Option<Input> { match hit_test(x,y,self.width) { Some(Control::Back)=>Some(Input::Back),Some(Control::Forward)=>Some(Input::Forward),Some(Control::Reload)=>Some(Input::Reload),Some(Control::Stop)=>Some(Input::Stop),Some(Control::Address)|None=>None } }
}
impl Window for DesktopWindow {
    fn present(&mut self, pixels:&[u32], width:u32, height:u32) {
        self.buffer.fill(0xff252526);
        let vw=self.width.min(width as usize); let vh=self.height.saturating_sub(37).min(height as usize);
        for y in 0..vh { let src=y*width as usize; let dst=(y+37)*self.width; self.buffer[dst..dst+vw].copy_from_slice(&pixels[src..src+vw]); }
        let _=self.window.update_with_buffer(&self.buffer,self.width,self.height);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn controls_are_pure() { assert_eq!(hit_test(10,10,800),Some(Control::Back)); assert_eq!(hit_test(80,10,800),Some(Control::Reload)); assert_eq!(hit_test(10,40,800),None); }
    #[test] fn address_editing_is_pure() { let mut a=AddressBar::new("ab"); a.move_left(); a.insert('x'); a.backspace(); assert_eq!(a.text,"ab"); a.move_right(); a.insert('c'); assert_eq!(a.text,"abc"); }
}
