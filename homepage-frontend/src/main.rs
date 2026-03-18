//! Homepage frontend

// Features
#![feature(
	try_blocks,
	thread_local,
	type_alias_impl_trait,
	stmt_expr_attributes,
	proc_macro_hygiene
)]

// Modules
mod pages;
mod util;

// Imports
use {
	self::util::NodeWithCssLink,
	app_error::AppError,
	dynatos_html::{ElementWithAttr, ElementWithClass, NodeWithChildren, NodeWithText, html},
	dynatos_html_reactive::{NodeWithDynChild, ObjectAttachContext},
	dynatos_reactive::SignalGetCloned,
	dynatos_router::Location,
	std::rc::Rc,
	tracing_subscriber::prelude::*,
	url::Url,
	web_sys::HtmlElement,
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
	body.attach_context(location.clone());

	// Build the backend url
	// TODO: Should this be reactive?
	let backend_url = location
		.get_cloned_raw()
		.join("backend/")
		.expect("Backend url was invalid");
	body.attach_context(BackendUrl(Rc::new(backend_url)));

	// And attach our app to the body
	body.with_child(
		html::div()
			.with_css_link("/css/app.css")
			.with_class("app")
			.with_child(self::render_nav())
			.with_child(html::div().with_class("main").with_dyn_child(self::render_route)),
	);

	Ok(())
}

fn render_nav() -> HtmlElement {
	let local = [
		("/", "Home"),
		("/projects", "Projects"),
		("/cv", "CV"),
		("/about-me", "About me"),
	];
	let external = [("https://gitea.filipejr.com", "Gitea")];

	html::nav()
		.with_css_link("/css/sidebar.css")
		.with_class("sidebar")
		.with_children([
			html::ul().with_class("local").with_children(
				local
					.iter()
					.map(|&(location, text)| html::li().with_child(dynatos_router::anchor(location).with_text(text)))
					.collect::<Vec<_>>(),
			),
			html::ul().with_class("external").with_children(
				external
					.iter()
					.map(|&(location, text)| {
						html::li().with_child(html::a().with_attr("href", location).with_text(text))
					})
					.collect::<Vec<_>>(),
			),
		])
}

fn render_route() -> Option<HtmlElement> {
	let location = dynatos_context::with_expect::<Location, _, _>(|location| location.get_cloned());

	tracing::debug!(%location, "Rendering route");
	match location.path().trim_end_matches('/') {
		"" => Some(pages::Home::new()),
		"/projects" => Some(pages::Projects::new()),
		"/cv" => Some(pages::CV::new()),
		"/about-me" => Some(pages::AboutMe::new()),
		_ => Some(pages::NotFound::new()),
	}
}
