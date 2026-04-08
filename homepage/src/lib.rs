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
	dynatos_web::{ElementWithClass, NodeWithChildren, html},
	dynatos_web_reactive::NodeWithDynChild,
	dynatos_web_router::Location,
	std::rc::Rc,
	url::Url,
};

#[derive(Clone)]
#[derive(derive_more::Deref)]
#[deref(forward)]
struct BackendUrl(Rc<Url>);

pub fn attach_to_body(body: &web_sys::HtmlElement, location: Location) {
	// Build the backend url
	// TODO: Should this be reactive?
	let backend_url = location
		.get_cloned_no_dep()
		.join("backend/")
		.expect("Backend url was invalid");
	let backend_url = BackendUrl(backend_url.into());

	// And attach our app to the body
	body.with_child(
		html::div()
			.with_class("app")
			.with_child(components::sidebar(&location, backend_url.clone()))
			.with_child(
				html::div()
					.with_class("body")
					.with_dyn_child(move || self::render_route(&location, backend_url.clone())),
			),
	);
}


fn render_route(location: &Location, backend_url: BackendUrl) -> web_sys::HtmlElement {
	let location = location.get_cloned();

	tracing::debug!(%location, "Rendering route");
	match location.path().trim_end_matches('/') {
		"" => pages::home(),
		"/projects" => pages::projects(backend_url),
		"/cv" => pages::cv(),
		"/about-me" => pages::about_me(),
		_ => pages::not_found(),
	}
}
