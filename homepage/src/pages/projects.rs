//! `/projects` page

// Imports
use {
	crate::BackendUrl,
	app_error::{AppError, Context},
	dynatos_loadable::{Loadable, LoadableSignal},
	dynatos_reactive::SignalBorrow,
	dynatos_web::{
		DynatosWebCtx,
		ElementWithClass,
		NodeWithChildren,
		NodeWithText,
		ObjectWithValue,
		html,
		types::HtmlElement,
	},
	dynatos_web_title::ObjectWithTitle,
	zutil_cloned::cloned,
};

pub fn projects(ctx: &DynatosWebCtx) -> HtmlElement {
	let backend_url = ctx.store().get::<BackendUrl>();

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

	#[cloned(ctx)]
	let projects = move || match projects.borrow() {
		Loadable::Empty => html::p(&ctx).with_value(ctx.wait_guard()).with_text("Loading..."),
		Loadable::Err(err) => html::pre(&ctx).with_text(format!("Unable to load projects:\n{err:?}")),
		Loadable::Loaded(projects) => html::ul(&ctx).with_class("projects").with_children(
			projects
				.projects
				.iter()
				.map(|project| dynatos_web::html_file!("homepage/html/pages/projects/project.html"))
				.collect::<Vec<_>>(),
		),
	};

	dynatos_web::html_file!("homepage/html/pages/projects.html").with_title(ctx, "Projects | Filipejr")
}
