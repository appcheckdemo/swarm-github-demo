use native_browser::{html, layout, paint, Browser};
use native_browser::model::NodeKind;

fn elements(s: &str) -> Vec<(String, String)> {
    let d = html::parse(s);
    d.root.children.iter().filter_map(|n| match &n.kind {
        NodeKind::Element(tag) => Some((tag.clone(), n.text())),
        _ => None,
    }).collect()
}

#[test]
fn parser_handles_nested_elements_and_malformed_close() {
    let d = html::parse("<main><h1>Title &amp; more</h1><p>body</main>");
    assert_eq!(d.root.children.len(), 1);
    assert_eq!(d.root.children[0].text(), "Title &amp; morebody");
    assert_eq!(d.root.children[0].children[0].text(), "Title &amp; more");
}

#[test]
fn parser_keeps_text_and_ignores_declarations() {
    let found = elements("<!doctype html><p>one</p>tail");
    assert_eq!(found, vec![("p".into(), "one".into())]);
    assert_eq!(html::parse("<p>one &lt; two</p>").root.children[0].text(), "one &lt; two");
}

#[test]
fn layout_has_stable_box_geometry_and_reflows_with_width() {
    let d = html::parse("<h1>A</h1><p>B</p><p>C</p>");
    let narrow = layout::layout(&d, 100);
    let wide = layout::layout(&d, 240);
    assert_eq!(narrow.items.len(), 3);
    assert_eq!(narrow.items[0].rect.x, 8);
    assert_eq!(narrow.items[0].rect.width, 84);
    assert!(narrow.items[1].rect.y > narrow.items[0].rect.y);
    assert_eq!(wide.items[0].rect.width, 224);
    assert_ne!(narrow.items[0].rect.width, wide.items[0].rect.width);
}

#[test]
fn paint_has_meaningful_colours_and_clips_at_viewport() {
    let d = html::parse("<h1>heading</h1><p>paragraph</p>");
    let list = layout::layout(&d, 80);
    let image = paint::paint(&list, 80, 20);
    assert_eq!(image.get_pixel(0, 0).0, [255, 255, 255, 255]);
    assert_eq!(image.get_pixel(8, 1).0, [35, 75, 130, 255]);
    assert_eq!(image.get_pixel(79, 19).0, [35, 75, 130, 255]);
}

#[test]
fn browser_builds_document_and_display_list_from_public_api() {
    let browser = Browser::from_html("<h1>hello</h1><p>world</p>", 320);
    assert_eq!(browser.width, 320);
    assert_eq!(browser.document.root.children.len(), 2);
    assert_eq!(browser.display_list.items[0].text.as_deref(), Some("hello"));
}
