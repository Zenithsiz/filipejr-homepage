//! Homepage frontend

// Features
#![feature(
	try_blocks,
	thread_local,
	type_alias_impl_trait,
	stmt_expr_attributes,
	proc_macro_hygiene,
	type_changing_struct_update
)]

// Modules
mod components;
mod pages;
mod util;

// Imports
use {
	app_error::AppError,
	dynatos_reactive::SignalGetCloned,
	dynatos_web::{ElementWithClass, NodeWithChildren, html},
	dynatos_web_reactive::NodeWithDynChild,
	dynatos_web_router::Location,
	std::rc::Rc,
	tracing_subscriber::prelude::*,
	url::Url,
};

fn main() {
	console_error_panic_hook::set_once();
	tracing_subscriber::registry()
		.with(
			tracing_subscriber::fmt::layer()
				.with_ansi(false)
				.without_time()
				.with_level(false)
				.with_writer(tracing_web::MakeWebConsoleWriter::new().with_pretty_level()),
		)
		.init();

	match self::run() {
		Ok(()) => tracing::info!("Successfully initialized"),
		Err(err) => tracing::error!("Unable to start: {err:?}"),
	}
}

#[derive(Clone)]
#[derive(derive_more::Deref)]
#[deref(forward)]
struct BackendUrl(Rc<Url>);

fn run() -> Result<(), AppError> {
	let window = web_sys::window().expect("Unable to get window");
	let document = window.document().expect("Unable to get document");
	let body = document.body().expect("Unable to get document body");

	let location = Location::new();

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
			.with_child(components::sidebar(location.clone(), backend_url.clone()))
			.with_child(
				html::div()
					.with_class("body")
					.with_dyn_child(move || self::render_route(&location, backend_url.clone())),
			),
	);

	Ok(())
}


fn render_route(location: &Location, backend_url: BackendUrl) -> Option<web_sys::HtmlElement> {
	let location = location.get_cloned();

	tracing::debug!(%location, "Rendering route");
	match location.path().trim_end_matches('/') {
		"" => Some(pages::home()),
		"/projects" => Some(pages::projects(backend_url)),
		"/cv" => Some(pages::cv()),
		"/about-me" => Some(pages::about_me()),
		_ => Some(pages::not_found()),
	}
}
