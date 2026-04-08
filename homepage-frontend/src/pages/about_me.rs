//! `/about-me` page

// Imports
use dynatos_web_title::ObjectWithTitle;

pub fn about_me() -> web_sys::HtmlElement {
	use homepage_dto::THIS_WEBSITE;
	dynatos_web::html_file!("homepage-frontend/html/pages/about-me.html").with_title("About me | Filipejr")
}
