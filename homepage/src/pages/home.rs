//! `/home` page

// Imports
use {
	dynatos_web::{DynatosWebCtx, types::HtmlElement},
	dynatos_web_title::ObjectWithTitle,
};

pub fn home(ctx: &DynatosWebCtx) -> HtmlElement {
	dynatos_web::html_file!("homepage/html/pages/home.html").with_title(ctx, "Home | Filipejr")
}
