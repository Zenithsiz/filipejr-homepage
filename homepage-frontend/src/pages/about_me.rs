//! `/about-me` page

// Imports
use dynatos_title::ObjectWithTitle;

#[dynatos_builder::builder]
pub fn AboutMe() -> web_sys::HtmlElement {
	use homepage::THIS_WEBSITE;
	dynatos_html::html_file!("homepage-frontend/pages/about-me.html").with_title("About me | Filipejr")
}
