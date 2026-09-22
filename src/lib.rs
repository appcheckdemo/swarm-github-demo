//! Native browser primitives with small replaceable interfaces.
pub mod model; pub mod dom; pub mod html; pub mod css; pub mod style; pub mod layout; pub mod paint; pub mod js; pub mod network; pub mod navigation; pub mod resources; pub mod events; pub mod platform; pub mod browser; pub mod headless;
pub use browser::Browser; pub use headless::render_png;
#[cfg(test)] mod tests { use super::*; #[test] fn parses_heading(){let d=html::parse("<h1>Hello</h1>"); assert_eq!(d.root.children[0].text(),"Hello");} #[test] fn paints_output(){let d=html::parse("<h1>Hi</h1><p>There</p>");let l=layout::layout(&d,320);let p=paint::paint(&l,320,120);assert_eq!(p.get_pixel(0,0).0,[255,255,255,255]);assert!(l.items.len()>=2);} }
