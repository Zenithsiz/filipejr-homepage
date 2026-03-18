//! `/cv` page

// Imports
use {
	dynatos_html::{ElementWithClass, EventTargetWithListener, NodeWithChildren, NodeWithText, ev, html},
	dynatos_html_reactive::ElementWithDynAttr,
	dynatos_reactive::{Signal, SignalGet, SignalSet},
	dynatos_title::ObjectWithTitle,
	strum::IntoEnumIterator,
	zutil_cloned::cloned,
};

#[dynatos_builder::builder]
pub fn CV() -> web_sys::HtmlElement {
	let cur_lang = Signal::new(Lang::En);

	let langs_selector = Lang::iter()
		.map(|lang| {
			let display = match lang {
				Lang::En => "English",
				Lang::Pt => "Português",
			};

			#[cloned(cur_lang)]
			html::button()
				.with_text(display)
				.with_event_listener::<ev!(click)>(move |_| cur_lang.set(lang))
		})
		.collect::<Vec<_>>();
	let lang_selector = html::div().with_class("lang-selector").with_children(langs_selector);

	let cvs = Lang::iter()
		.map(|lang| {
			let src = format!("/backend/cv.pdf?lang={lang}");

			#[cloned(cur_lang)]
			dynatos_html::html_file!("homepage-frontend/pages/cv/pdf.html")
				.with_dyn_attr_if("hidden", move || cur_lang.get() != lang)
		})
		.collect::<Vec<_>>();

	let cvs = html::div().with_class("cvs").with_children(cvs);

	dynatos_html::html_file!("homepage-frontend/pages/cv.html").with_title("CV | Filipejr")
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[derive(derive_more::Display)]
#[derive(strum::EnumIter)]
enum Lang {
	#[display("en")]
	En,
	#[display("pt")]
	Pt,
}
