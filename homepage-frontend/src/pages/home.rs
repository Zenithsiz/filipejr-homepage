//! `/home` page

// Imports
use dynatos_title::ObjectWithTitle;

#[dynatos_builder::builder]
pub fn Home() -> web_sys::HtmlElement {
	dynatos_html::html_file!("homepage-frontend/html/pages/home.html").with_title("Home | Filipejr")
}
