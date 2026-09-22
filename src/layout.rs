//! Deterministic, dependency-free layout.  This is deliberately conservative: when
//! a caller has no CSS information we still produce useful, stable geometry.
use crate::{dom::{Node, NodeKind}, model::{Point, Rect, Size}, style::{ComputedStyle, Display}};

#[derive(Clone, Debug)]
pub struct LayoutBox { pub rect: Rect, pub node: Option<crate::model::NodeId>, pub children: Vec<LayoutBox> }

const CHAR: f32 = 8.0;
const LINE: f32 = 16.0;
const LIMIT: f32 = 1_000_000.0;

fn number(node: &Node, name: &str) -> Option<f32> {
    node.attributes.iter().find(|(n, _)| n.eq_ignore_ascii_case(name))
        .and_then(|(_, v)| v.trim_end_matches("px").parse::<f32>().ok())
        .map(|v| v.max(0.0).min(LIMIT))
}
fn attr(node: &Node, name: &str) -> Option<&str> {
    node.attributes.iter().find(|(n, _)| n.eq_ignore_ascii_case(name)).map(|(_,v)| v.as_str())
}
fn inline_tag(tag: &str) -> bool { matches!(tag, "a"|"abbr"|"b"|"br"|"code"|"em"|"i"|"img"|"input"|"label"|"small"|"span"|"strong"|"sub"|"sup") }
fn text_height(text: &str, width: f32) -> f32 {
    let cols = (width.max(CHAR) / CHAR).floor().max(1.0) as usize;
    let mut lines = 1usize; let mut col = 0usize;
    for word in text.split_whitespace() {
        let n = word.chars().count();
        if col != 0 && col + 1 + n > cols { lines += 1; col = n.min(cols); }
        else { col += if col == 0 { n } else { n + 1 }; if col > cols { lines += (col-1)/cols; col = (col-1)%cols + 1; } }
    }
    (lines as f32 * LINE).min(LIMIT)
}

/// Lay out a tree in normal flow.  Coordinates are viewport-relative and never NaN.
pub fn layout(node: &Node, style: &ComputedStyle, viewport: Size) -> LayoutBox {
    let width = viewport.width.max(0.0).min(LIMIT);
    layout_node(node, style, Point { x: 0.0, y: 0.0 }, width)
}

fn layout_node(node: &Node, style: &ComputedStyle, origin: Point, available: f32) -> LayoutBox {
    if style.display == Display::None { return LayoutBox { rect: Rect { origin, size: Size::default() }, node: Some(node.id), children: vec![] }; }
    let pad = number(node, "padding").unwrap_or(0.0);
    let border = number(node, "border").unwrap_or(0.0);
    let margin = number(node, "margin").unwrap_or(0.0);
    let outer = style.width.or_else(|| number(node, "width")).unwrap_or((available - 2.0*margin).max(0.0)).min(available.max(0.0));
    let content = (outer - 2.0*(pad + border)).max(0.0);
    let tag = match &node.kind { NodeKind::Element { tag } => tag.to_ascii_lowercase(), _ => String::new() };
    let mut children = Vec::new(); let mut y = origin.y + pad + border; let mut max_x = origin.x;
    for child in &node.children {
        let child_style = ComputedStyle { display: if matches!(child.kind, NodeKind::Text(_)) { Display::Inline } else if let NodeKind::Element { ref tag } = child.kind { if inline_tag(&tag.to_ascii_lowercase()) { Display::Inline } else { Display::Block } } else { Display::None }, ..ComputedStyle::default() };
        let h = match &child.kind { NodeKind::Text(t) => text_height(t, content), _ => 0.0 };
        let mut b = if h > 0.0 { LayoutBox { rect: Rect { origin: Point { x: origin.x+pad+border, y }, size: Size { width: content, height: h } }, node: Some(child.id), children: vec![] } } else { layout_node(child, &child_style, Point { x: origin.x+pad+border, y }, content) };
        y += b.rect.size.height; max_x = max_x.max(b.rect.origin.x + b.rect.size.width); children.push(b);
    }
    let natural = (y - origin.y) + pad + border;
    let height = style.height.or_else(|| number(node, "height")).unwrap_or(if node.children.is_empty() && !tag.is_empty() { if tag == "img" { number(node,"height").unwrap_or(16.0) } else { LINE } } else { natural }).max(0.0).min(LIMIT);
    let _ = (max_x, attr(node, "position")); // reserved for future positioned containing blocks
    LayoutBox { rect: Rect { origin: Point { x: origin.x + margin, y: origin.y + margin }, size: Size { width: outer, height } }, node: Some(node.id), children }
}

#[cfg(test)]
mod tests {
    use super::*; use crate::{dom::Node, model::{NodeId, Size}};
    #[test] fn wraps_text_deterministically() { let mut n=Node::element(NodeId(1),"p"); n.append(Node { id:NodeId(2), kind:NodeKind::Text("one two three four".into()), children:vec![], attributes:vec![] }); let a=layout(&n,&ComputedStyle::default(),Size{width:32.0,height:100.0}); assert!(a.children[0].rect.size.height >= 64.0); }
    #[test] fn nested_box_model_is_nonnegative() { let mut n=Node::element(NodeId(1),"div"); n.attributes=vec![("padding".into(),"4".into()),("border".into(),"2".into())]; let a=layout(&n,&ComputedStyle::default(),Size{width:20.0,height:20.0}); assert!(a.rect.size.width>=0.0); }
}
