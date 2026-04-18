//! Not found page

// Imports
use {
	dynatos_web::{DynatosWebCtx, NodeWithText, html, types::HtmlElement},
	dynatos_web_title::ObjectWithTitle,
};

pub fn not_found(ctx: &DynatosWebCtx, page: &str) -> HtmlElement {
	html::p(ctx)
		.with_title(ctx, "Not found | Filipejr")
		.with_text(format!("Unknown page: {page:?}"))
}
