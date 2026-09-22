//! Deterministic PNG rendering entry point.
use std::path::Path;use crate::{html,layout,paint};pub fn render_png(s:&str,p:impl AsRef<Path>,w:u32,h:u32)->image::ImageResult<()>{paint::paint(&layout::layout(&html::parse(s),w),w,h).save(p)}
