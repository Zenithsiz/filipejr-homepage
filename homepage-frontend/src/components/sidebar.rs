//! Sidebar

// Imports
use {
	crate::util::NodeWithCssLink,
	dynatos_html::{ElementWithAttr, ElementWithClass, NodeWithChildren, NodeWithText, html},
};

#[dynatos_builder::builder]
pub fn Sidebar() -> web_sys::HtmlElement {
	let local = [
		("/", "Home"),
		("/projects", "Projects"),
		("/cv", "CV"),
		("/about-me", "About me"),
	];
	let external = [("https://gitea.filipejr.com", "Gitea")];

	html::nav()
		.with_css_link("/css/components/sidebar.css")
		.with_class("sidebar")
		.with_children([
			html::ul().with_class("local").with_children(
				local
					.iter()
					.map(|&(location, text)| html::li().with_child(dynatos_router::anchor(location).with_text(text)))
					.collect::<Vec<_>>(),
			),
			html::ul().with_class("external").with_children(
				external
					.iter()
					.map(|&(location, text)| {
						html::li().with_child(html::a().with_attr("href", location).with_text(text))
					})
					.collect::<Vec<_>>(),
			),
		])
}
