//! Native browser building blocks. This crate deliberately contains no web parser or JS engine.
pub mod model; pub mod dom; pub mod html; pub mod css; pub mod style; pub mod layout;
pub mod paint; pub mod resources; pub mod js; pub mod events; pub mod network;
pub mod navigation; pub mod browser; pub mod headless; pub mod platform;

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn document_has_stable_root() { assert_eq!(dom::Document::default().root.id, model::NodeId(0)); }
    #[test] fn navigation_records_url() { let mut n=navigation::Navigation::default(); n.navigate("about:blank"); assert_eq!(n.current.as_deref(),Some("about:blank")); }
    #[test] fn null_transport_is_safe() { assert!(network::Transport::fetch(&network::NullTransport, network::Request{url:"x".into()}).is_err()); }
}
