//! Sidebar

// Imports
use {
	crate::{BackendUrl, util::NodeWithCssLink},
	app_error::{AppError, Context},
	dynatos_html::{ElementWithAttr, ElementWithClass, NodeWithChildren, NodeWithText, html},
	dynatos_html_reactive::NodeWithDynChildren,
	dynatos_loadable::{Loadable, LoadableSignal},
	dynatos_reactive::SignalBorrow,
};

#[dynatos_builder::builder]
pub fn Sidebar() -> web_sys::HtmlElement {
	let local_links = [
		("/", "Home"),
		("/projects", "Projects"),
		("/cv", "CV"),
		("/about-me", "About me"),
	];
	let external_links = LoadableSignal::new(|| async move {
		let backend_url = dynatos_context::expect_cloned::<BackendUrl>();
		let external_links_url = backend_url.join("external-links").context("Unable to create url")?;
		let external_links = reqwest::get(external_links_url)
			.await
			.context("Unable to get external links")?
			.json::<homepage::ExternalLinks>()
			.await
			.context("Unable to parse external links")?;

		Ok::<_, AppError>(external_links)
	});

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

	html::nav()
		.with_css_link("/css/components/sidebar.css")
		.with_class("sidebar")
		.with_children([
			html::ul().with_class("local").with_children(
				local_links
					.iter()
					.map(|&(location, text)| html::li().with_child(dynatos_router::anchor(location).with_text(text)))
					.collect::<Vec<_>>(),
			),
			html::ul().with_class("external").with_dyn_children(external_links),
		])
}
