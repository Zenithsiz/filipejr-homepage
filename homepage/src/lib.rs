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
	dynatos_web::{DynatosWebCtx, ElementWithClass, NodeWithChildren, html},
	dynatos_web_reactive::NodeWithDynChild,
	dynatos_web_router::Location,
	std::rc::Rc,
	url::Url,
	zutil_cloned::cloned,
};

#[derive(Clone)]
#[derive(derive_more::Deref)]
#[deref(forward)]
struct BackendUrl(Rc<Url>);

pub fn attach_to_body(ctx: &DynatosWebCtx, location: Location) {
	// Build the backend url
	// TODO: Should this be reactive?
	let backend_url = location
		.get_cloned_no_dep()
		.join("backend/")
		.expect("Backend url was invalid");
	let backend_url = BackendUrl(backend_url.into());

	// And attach our app to the body
	ctx.body().with_child(
		html::div(ctx)
			.with_class("app")
			.with_child(components::sidebar(ctx, &location, backend_url.clone()))
			.with_child(html::div(ctx).with_class("body").with_dyn_child(
				ctx,
				#[cloned(ctx)]
				move || self::render_route(&ctx, &location, backend_url.clone()),
			)),
	);
}


fn render_route(ctx: &DynatosWebCtx, location: &Location, backend_url: BackendUrl) -> web_sys::HtmlElement {
	let location = location.get_cloned();

	tracing::debug!(%location, "Rendering route");
	match location.path().trim_end_matches('/') {
		"" => pages::home(ctx),
		"/projects" => pages::projects(ctx, backend_url),
		"/cv" => pages::cv(ctx),
		"/about-me" => pages::about_me(ctx),
		_ => pages::not_found(ctx),
	}
}
