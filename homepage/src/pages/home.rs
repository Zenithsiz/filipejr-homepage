//! `/home` page

// Imports
use {dynatos_web::DynatosWebCtx, dynatos_web_title::ObjectWithTitle};

pub fn home(ctx: &DynatosWebCtx) -> web_sys::HtmlElement {
	dynatos_web::html_file!("homepage/html/pages/home.html").with_title(ctx, "Home | Filipejr")
}
