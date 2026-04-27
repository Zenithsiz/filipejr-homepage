//! Homepage frontend

// Features
#![feature(stmt_expr_attributes, proc_macro_hygiene)]

// Modules
mod components;
mod pages;
mod util;

// Imports
use {
	dynatos_reactive::SignalGetCloned,
	dynatos_sync_types::RcPtr,
	dynatos_web::{DynatosWebCtx, ElementWithClass, NodeAddChildren, NodeWithChildren, html, types::HtmlElement},
	dynatos_web_reactive::NodeWithDynChild,
	dynatos_web_router::LocationSignal,
	url::Url,
	zutil_cloned::cloned,
};

// TODO: `dynatos_sync_types` isn't public API so we should make our own.
#[derive(Clone)]
#[derive(derive_more::Deref)]
#[deref(forward)]
struct BackendUrl(RcPtr<Url>);

pub fn attach(ctx: &DynatosWebCtx) {
	let location = LocationSignal::new(ctx);

	// Build the backend url
	// TODO: Should this be reactive?
	let backend_url = location
		.get_cloned_no_dep()
		.join("backend/")
		.expect("Backend url was invalid");
	let backend_url = BackendUrl(backend_url.into());

	ctx.store().set(location);
	ctx.store().set(backend_url);

	ctx.head().add_children([
		html!(r#"<link href="/css/default.css" rel="stylesheet" />"#),
		html!(r#"<link href="/css/colors.css" rel="stylesheet" />"#),
		html!(r#"<link href="/css/a.css" rel="stylesheet" />"#),
		html!(r#"<link href="/css/app.css" rel="stylesheet" />"#),
		html!(r#"<meta charset="UTF-8" />"#),
		html!(r#"<meta name="viewport" content="width=device-width, initial-scale=1.0" />"#),
		html!(r#"<title>Filipejr</title>"#),
	]);

	// And attach our app to the body
	ctx.body().with_child(
		html::div(ctx)
			.with_class("app")
			.with_child(components::sidebar(ctx))
			.with_child(html::div(ctx).with_class("body").with_dyn_child(
				ctx,
				#[cloned(ctx)]
				move || self::render_route(&ctx),
			)),
	);
}


fn render_route(ctx: &DynatosWebCtx) -> HtmlElement {
	let location = ctx.store().get::<LocationSignal>().get_cloned();

	tracing::debug!(%location, "Rendering route");
	match location.path().trim_end_matches('/') {
		"" => pages::home(ctx),
		"/projects" => pages::projects(ctx),
		"/cv" => pages::cv(ctx),
		"/about-me" => pages::about_me(ctx),
		page => pages::not_found(ctx, page),
	}
}
