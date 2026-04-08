//! Sidebar

// Imports
use {
	crate::BackendUrl,
	app_error::{AppError, Context},
	dynatos_loadable::{Loadable, LoadableSignal},
	dynatos_reactive::SignalBorrow,
	dynatos_web::{ElementWithAttr, NodeWithChildren, NodeWithText, html},
	dynatos_web_router::Location,
	zutil_cloned::cloned,
};

pub fn sidebar(location: &Location, backend_url: BackendUrl) -> web_sys::HtmlElement {
	let local_links = [
		("/", "Home"),
		("/projects", "Projects"),
		("/cv", "CV"),
		("/about-me", "About me"),
	];
	let external_links = LoadableSignal::new(move || {
		#[cloned(backend_url)]
		async move {
			let external_links_url = backend_url.join("external-links").context("Unable to create url")?;
			let external_links = reqwest::get(external_links_url)
				.await
				.context("Unable to get external links")?
				.json::<homepage_dto::ExternalLinks>()
				.await
				.context("Unable to parse external links")?;

			Ok::<_, AppError>(external_links)
		}
	});

	let local_links = local_links
		.iter()
		.map(|&(new_location, text)| {
			html::li().with_child(dynatos_web_router::anchor(location.clone(), new_location).with_text(text))
		})
		.collect::<Vec<_>>();

	let external_links = move || match external_links.borrow() {
		Loadable::Empty => vec![html::p().with_text("Loading...")],
		Loadable::Err(err) => vec![html::pre().with_text(format!("Unable to load projects:\n{err:?}"))],
		Loadable::Loaded(external_links) => external_links
			.links
			.iter()
			.map(|link| {
				html::li().with_child(
					html::a()
						.with_attr("href", &link.location)
						.with_text(link.text.as_str()),
				)
			})
			.collect::<Vec<_>>(),
	};

	dynatos_web::html_file!("homepage-frontend/html/components/sidebar.html")
}
