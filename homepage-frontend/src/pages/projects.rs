//! `/projects` page

// Imports
use {
	crate::BackendUrl,
	app_error::{AppError, Context},
	dynatos_html::{ElementWithClass, NodeWithChildren, NodeWithText, html},
	dynatos_loadable::{Loadable, LoadableSignal},
	dynatos_reactive::SignalBorrow,
	dynatos_title::ObjectWithTitle,
};

#[dynatos_builder::builder]
pub fn Projects() -> web_sys::HtmlElement {
	let projects = LoadableSignal::new(|| async move {
		let backend_url = dynatos_context::expect_cloned::<BackendUrl>();
		let projects_url = backend_url.join("projects").context("Unable to create url")?;
		let projects = reqwest::get(projects_url)
			.await
			.context("Unable to get project")?
			.json::<homepage::Projects>()
			.await
			.context("Unable to parse projects")?;

		Ok::<_, AppError>(projects)
	});

	let projects = move || match projects.borrow() {
		Loadable::Empty => html::p().with_text("Loading..."),
		Loadable::Err(err) => html::pre().with_text(format!("Unable to load projects:\n{err:?}")),
		Loadable::Loaded(projects) => html::ul().with_class("projects").with_children(
			projects
				.projects
				.iter()
				.map(|project| dynatos_html::html_file!("homepage-frontend/pages/projects/project.html"))
				.collect::<Vec<_>>(),
		),
	};

	dynatos_html::html_file!("homepage-frontend/pages/projects.html").with_title("Projects | Filipejr")
}
