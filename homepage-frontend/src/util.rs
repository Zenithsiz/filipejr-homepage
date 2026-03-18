//! Utilities

// Imports
use dynatos_html::{ElementWithAttr, NodeAddChildren, html};

/// Returns a `<link href="<href>" rel="stylesheet"/>`
// TODO: Move this to `dynatos`?
#[must_use]
pub fn css_link(href: &str) -> web_sys::HtmlElement {
	html::link().with_attr("href", href).with_attr("rel", "stylesheet")
}

#[extend::ext(name = NodeAddCssLink)]
pub impl web_sys::Node {
	fn add_css_link(&self, href: &str) {
		self.add_child(self::css_link(href));
	}
}

#[extend::ext(name = NodeWithCssLink)]
pub impl<N: AsRef<web_sys::Node>> N {
	fn with_css_link(self, href: &str) -> Self {
		self.as_ref().add_css_link(href);
		self
	}
}
