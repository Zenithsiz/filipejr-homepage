//! Not found page

// Imports
use {
	dynatos_html::{NodeWithText, html},
	dynatos_title::ObjectWithTitle,
};

#[dynatos_builder::builder]
pub fn NotFound() -> web_sys::HtmlElement {
	html::p().with_title("Not found | Filipejr").with_text("Unknown page")
}
