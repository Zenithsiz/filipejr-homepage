//! Not found page

// Imports
use {
	dynatos_web::{NodeWithText, html},
	dynatos_web_title::ObjectWithTitle,
};

pub fn not_found() -> web_sys::HtmlElement {
	html::p().with_title("Not found | Filipejr").with_text("Unknown page")
}
