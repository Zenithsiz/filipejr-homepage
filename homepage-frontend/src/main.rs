//! Homepage frontend

// Imports
use {
	app_error::{AppError, Context},
	dynatos_web::DynatosWebCtx,
	dynatos_web_router::Location,
	tracing_subscriber::prelude::*,
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

fn run() -> Result<(), AppError> {
	let ctx = DynatosWebCtx::new().context("Unable to build dynatos web context")?;
	let location = Location::new(&ctx);

	homepage::attach_to_body(&ctx, location);

	Ok(())
}
