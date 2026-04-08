//! `/about-me` page

// Imports
use {dynatos_web::DynatosWebCtx, dynatos_web_title::ObjectWithTitle};

pub fn about_me(ctx: &DynatosWebCtx) -> web_sys::HtmlElement {
	use homepage_dto::THIS_WEBSITE;
	dynatos_web::html_file!("homepage/html/pages/about-me.html").with_title(ctx, "About me | Filipejr")
}
