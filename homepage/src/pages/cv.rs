//! `/cv` page

// Imports
use {
	dynatos_reactive::{Signal, SignalGet, SignalSet},
	dynatos_web::{
		DynatosWebCtx,
		ElementWithClass,
		EventTargetWithListener,
		NodeWithChildren,
		NodeWithText,
		ev,
		html,
		types::HtmlElement,
	},
	dynatos_web_reactive::ElementWithDynAttr,
	dynatos_web_title::ObjectWithTitle,
	strum::IntoEnumIterator,
	zutil_cloned::cloned,
};

pub fn cv(ctx: &DynatosWebCtx) -> HtmlElement {
	let cur_lang = Signal::new(Lang::En);

	let langs_selector = Lang::iter()
		.map(|lang| {
			let display = match lang {
				Lang::En => "English",
				Lang::Pt => "Português",
			};

			#[cloned(cur_lang)]
			html::button(ctx)
				.with_text(display)
				.with_event_listener::<ev!(click)>(ctx, move |_| cur_lang.set(lang))
		})
		.collect::<Vec<_>>();
	let lang_selector = html::div(ctx).with_class("lang-selector").with_children(langs_selector);

	let cvs = Lang::iter()
		.map(|lang| {
			let src = format!("/backend/cv.pdf?lang={lang}");

			#[cloned(cur_lang)]
			dynatos_web::html_file!("homepage/html/pages/cv/pdf.html")
				.with_dyn_attr_if("hidden", move || cur_lang.get() != lang)
		})
		.collect::<Vec<_>>();

	let cvs = html::div(ctx).with_class("cvs").with_children(cvs);

	dynatos_web::html_file!("homepage/html/pages/cv.html").with_title(ctx, "CV | Filipejr")
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
