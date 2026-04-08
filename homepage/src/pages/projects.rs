//! `/projects` page

// Imports
use {
	crate::BackendUrl,
	app_error::{AppError, Context},
	dynatos_loadable::{Loadable, LoadableSignal},
	dynatos_reactive::SignalBorrow,
	dynatos_web::{ElementWithClass, NodeWithChildren, NodeWithText, html},
	dynatos_web_title::ObjectWithTitle,
	zutil_cloned::cloned,
};

pub fn projects(backend_url: BackendUrl) -> web_sys::HtmlElement {
	let projects = LoadableSignal::new(move || {
		#[cloned(backend_url)]
		async move {
			let projects_url = backend_url.join("projects").context("Unable to create url")?;
			let projects = reqwest::get(projects_url)
				.await
				.context("Unable to get project")?
				.json::<homepage_dto::Projects>()
				.await
				.context("Unable to parse projects")?;

			Ok::<_, AppError>(projects)
		}
	});

	let projects = move || match projects.borrow() {
		Loadable::Empty => html::p().with_text("Loading..."),
		Loadable::Err(err) => html::pre().with_text(format!("Unable to load projects:\n{err:?}")),
		Loadable::Loaded(projects) => html::ul().with_class("projects").with_children(
			projects
				.projects
				.iter()
				.map(|project| dynatos_web::html_file!("homepage/html/pages/projects/project.html"))
				.collect::<Vec<_>>(),
		),
	};

	dynatos_web::html_file!("homepage/html/pages/projects.html").with_title("Projects | Filipejr")
}
