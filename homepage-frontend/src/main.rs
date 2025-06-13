//! Homepage frontend

// Features
#![feature(try_blocks, thread_local, type_alias_impl_trait)]

// Imports
use {
	dynatos::{NodeWithDynChild, ObjectAttachContext},
	dynatos_html::{html, ElementWithAttr, ElementWithClass, NodeWithChildren, NodeWithText},
	dynatos_loadable::{Loadable, LoadableSignal},
	dynatos_reactive::{SignalBorrow, SignalGetCloned},
	dynatos_router::Location,
	dynatos_title::ObjectWithTitle,
	homepage::Project,
	std::rc::Rc,
	tracing_subscriber::prelude::*,
	url::Url,
	web_sys::HtmlElement,
	zutil_app_error::{AppError, AppErrorContext},
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
			.with_class("app")
			.with_child(self::render_nav())
			.with_child(html::div().with_class("main").with_dyn_child(self::render_route)),
	);

	Ok(())
}

fn render_nav() -> HtmlElement {
	let local = [("/", "Home"), ("/projects", "Projects"), ("/about-me", "About me")];
	let external = [("https://gitea.filipejr.com", "Gitea")];

	html::nav().with_class("sidebar").with_children([
		html::ul().with_class("local").with_children(
			local
				.iter()
				.map(|&(location, text)| html::li().with_child(dynatos_router::anchor(location).with_text(text)))
				.collect::<Vec<_>>(),
		),
		html::ul().with_class("external").with_children(
			external
				.iter()
				.map(|&(location, text)| html::li().with_child(html::a().with_attr("href", location).with_text(text)))
				.collect::<Vec<_>>(),
		),
	])
}

fn render_route() -> Option<HtmlElement> {
	let location = dynatos_context::with_expect::<Location, _, _>(|location| location.get_cloned());

	tracing::info!(%location, "Rendering route");
	match location.path().trim_end_matches('/') {
		"" => Some(Home::new()),
		"/projects" => Some(Projects::new()),
		"/about-me" => Some(AboutMe::new()),
		_ => Some(NotFound::new()),
	}
}


#[dynatos_builder::builder]
fn Home() -> web_sys::HtmlElement {
	dynatos_html::html_file!("homepage-frontend/pages/home.html").with_title("Home | Filipejr")
}

#[dynatos_builder::builder]
fn Projects() -> web_sys::HtmlElement {
	let projects = LoadableSignal::new(|| async move {
		let backend_url = dynatos_context::expect_cloned::<BackendUrl>();
		let projects_url = backend_url.join("projects").context("Unable to create url")?;
		let projects = reqwest::get(projects_url)
			.await
			.context("Unable to get project")?
			.json::<Vec<Project>>()
			.await
			.context("Unable to parse projects")?;

		Ok::<_, AppError>(projects)
	});

	html::div()
		.with_class("projects")
		.with_title("Projects | Filipejr")
		.with_dyn_child(move || match projects.borrow() {
			Loadable::Empty => html::p().with_text("Loading..."),
			Loadable::Err(err) => html::pre().with_text(format!("Unable to load projects:\n{err:?}")),
			Loadable::Loaded(projects) => html::ul().with_class("project").with_children(
				projects
					.iter()
					.map(|project| {
						html::li().with_child(
							html::a()
								.with_attr("href", &project.link)
								.with_text(project.name.as_str()),
						)
					})
					.collect::<Vec<_>>(),
			),
		})
}

#[dynatos_builder::builder]
fn AboutMe() -> web_sys::HtmlElement {
	use homepage::THIS_WEBSITE;
	dynatos_html::html_file!("homepage-frontend/pages/about-me.html").with_title("About me | Filipejr")
}

#[dynatos_builder::builder]
fn NotFound() -> web_sys::HtmlElement {
	html::p().with_title("Not found | Filipejr").with_text("Unknown page")
}
