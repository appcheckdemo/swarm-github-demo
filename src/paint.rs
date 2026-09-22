//! Deterministic software painter.
use crate::model::*;use image::{Rgba,RgbaImage};pub fn paint(l:&DisplayList,w:u32,h:u32)->RgbaImage{let mut i=RgbaImage::from_pixel(w,h,Rgba([255,255,255,255]));for x in &l.items{for y in x.rect.y..(x.rect.y+x.rect.height).min(h){for xx in x.rect.x..(x.rect.x+x.rect.width).min(w){i.put_pixel(xx,y,Rgba(x.color))}}}i}
