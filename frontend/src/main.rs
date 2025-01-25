//! Router example

// Features
#![feature(try_blocks, thread_local)]

// Imports
use {
	dynatos::{NodeWithDynChild, ObjectWithContext},
	dynatos_html::{html, ElementWithAttr, ElementWithClass, NodeWithChildren, NodeWithText},
	dynatos_reactive::SignalGetCloned,
	dynatos_router::Location,
	dynatos_title::ObjectWithTitle,
	tracing_subscriber::prelude::*,
	web_sys::Element,
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

fn run() -> Result<(), anyhow::Error> {
	let window = web_sys::window().expect("Unable to get window");
	let document = window.document().expect("Unable to get document");
	let body = document.body().expect("Unable to get document body");

	let location = Location::new();
	body.with_context(location).with_child(
		html::div()
			.with_class("app")
			.with_child(self::render_nav())
			.with_child(html::div().with_class("main").with_dyn_child(self::render_route)),
	);

	Ok(())
}

fn render_nav() -> Element {
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

fn render_route() -> Option<Element> {
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
fn Home() -> web_sys::Element {
	dynatos_html::html_file!("frontend/pages/home.html").with_title("Home | Filipejr")
}

#[dynatos_builder::builder]
fn Projects() -> web_sys::Element {
	let projects = [
		(
			"[ddw3] Digimon world 2003 decompilation",
			"https://gitea.filipejr.com/zenithsiz/ddw3",
		),
		(
			"[zbuild] Make-like build system",
			"https://gitea.filipejr.com/zenithsiz/zbuild",
		),
		(
			"[zsw] Zenithsiz's scrolling wallpaper",
			"https://gitea.filipejr.com/zenithsiz/zsw",
		),
		(
			"🚧 [dynatos] Rust web framework",
			"https://gitea.filipejr.com/zenithsiz/dynatos",
		),
		("[filipejr-homepage] This page", THIS_WEBSITE),
	];

	html::div()
		.with_class("projects")
		.with_title("Projects | Filipejr")
		.with_child(
			html::ul().with_class("project").with_children(
				projects
					.iter()
					.map(|&(name, link)| html::li().with_child(html::a().with_attr("href", link).with_text(name)))
					.collect::<Vec<_>>(),
			),
		)
}

/// This website's source page
const THIS_WEBSITE: &str = "https://gitea.filipejr.com/zenithsiz/filipejr-homepage";

#[dynatos_builder::builder]
fn AboutMe() -> web_sys::Element {
	dynatos_html::html_file!("frontend/pages/about-me.html").with_title("About me | Filipejr")
}

#[dynatos_builder::builder]
fn NotFound() -> web_sys::Element {
	html::p().with_title("Not found | Filipejr").with_text("Unknown page")
}
