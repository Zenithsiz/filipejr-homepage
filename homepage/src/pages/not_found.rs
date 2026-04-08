//! Not found page

// Imports
use {
	dynatos_web::{DynatosWebCtx, NodeWithText, html},
	dynatos_web_title::ObjectWithTitle,
};

pub fn not_found(ctx: &DynatosWebCtx) -> web_sys::HtmlElement {
	html::p(ctx)
		.with_title(ctx, "Not found | Filipejr")
		.with_text("Unknown page")
}
