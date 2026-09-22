use std::fs;
use native_browser::{headless, navigation};

#[test]
fn navigation_accepts_file_and_absolute_urls() {
    let file = navigation::parse("/tmp/page.html").unwrap();
    assert_eq!(file.scheme(), "file");
    let web = navigation::parse("https://example.invalid/a?q=1#frag").unwrap();
    assert_eq!(web.path(), "/a");
    assert_eq!(web.fragment(), Some("frag"));
}

#[test]
fn headless_render_writes_decodable_pixels() {
    let path = std::env::temp_dir().join(format!("native-browser-{}.png", std::process::id()));
    headless::render_png("<h1>hello</h1><p>world</p>", &path, 96, 64).unwrap();
    let image = image::open(&path).unwrap().to_rgba8();
    assert_eq!(image.dimensions(), (96, 64));
    assert_eq!(image.get_pixel(8, 1).0, [35, 75, 130, 255]);
    assert_ne!(image.get_pixel(8, 1).0, image.get_pixel(0, 0).0);
    fs::remove_file(path).unwrap();
}

#[test]
fn headless_render_recovers_after_empty_document() {
    let path = std::env::temp_dir().join(format!("native-browser-empty-{}.png", std::process::id()));
    headless::render_png("", &path, 17, 11).unwrap();
    let image = image::open(&path).unwrap();
    assert_eq!(image.dimensions(), (17, 11));
    assert_eq!(image.to_rgba8().get_pixel(0, 0).0, [255, 255, 255, 255]);
    fs::remove_file(path).unwrap();
}
