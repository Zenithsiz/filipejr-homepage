//! Homepage backend

// Features
#![feature(proc_macro_hygiene, yeet_expr)]

// Imports
use {
	app_error::{AppError, Context, ensure},
	axum::{
		Json,
		extract,
		http::StatusCode,
		response::{IntoResponse, Redirect},
	},
	core::time::Duration,
	homepage_dto::{ExternalLinks, Projects},
	std::{
		io,
		net::{IpAddr, Ipv4Addr, SocketAddr},
		path::{Path, PathBuf},
		sync::Arc,
	},
	tokio::fs,
	url::Url,
};

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
struct Config {
	/// Port
	port: u16,

	/// Resources directory
	resources: PathBuf,

	/// Location url.
	///
	/// Used during SSR.
	location: Url,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			port:      8081,
			resources: PathBuf::from("resources/"),
			location:  "http://localhost:8081".parse().expect("Should be a valid url"),
		}
	}
}

struct State {
	config: Config,
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
	let _logger = zutil_logger::Logger::new();

	// Read the config file
	let config_file = Path::new("config.toml");
	let config = match fs::read_to_string(&config_file).await {
		Ok(config) => toml::from_str::<Config>(&config).context("Unable to parse config")?,
		Err(err) if err.kind() == io::ErrorKind::NotFound => {
			let config = Config::default();
			let config_str = toml::to_string_pretty(&config).context("Unable to serialize config")?;
			if let Err(err) = fs::write(config_file, &config_str).await {
				tracing::warn!(?config_file, ?err, "Unable to write default config file");
			}

			config
		},
		Err(err) => return Err(AppError::new(&err).context("Unable to read config file")),
	};
	tracing::debug!("Configuration: {config:?}");

	let state = State { config };
	let state = Arc::new(state);

	// Then build the app
	let app = {
		use axum::routing::{any, get};
		let ssr_router = dynatos_web_ssr_server::axum::router(
			homepage::attach,
			state.config.location.clone(),
			Duration::from_hours(1),
		);

		axum::Router::new()
			.route("/backend/{*path}", any(self::redirect_backend))
			.route("/projects", get(self::projects))
			.route("/external-links", get(self::external_links))
			.route("/cv.pdf", get(self::cv))
			.with_state(Arc::clone(&state))
			.nest("/ssr/", ssr_router)
	};

	let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), state.config.port);
	let listener = tokio::net::TcpListener::bind(addr)
		.await
		.context("Unable to create tcp listener")?;
	axum::serve(listener, app)
		.await
		.context("Unable to start http server")?;

	Ok(())
}

#[axum::debug_handler]
async fn redirect_backend(uri: extract::OriginalUri) -> Result<Redirect, ReqError> {
	let uri = uri.to_string();
	let uri = uri.strip_prefix("/backend").context("Missing prefix")?;

	Ok(Redirect::permanent(uri))
}

async fn external_links(extract::State(state): extract::State<Arc<State>>) -> Result<Json<ExternalLinks>, ReqError> {
	let external_links_path = state.config.resources.join("external-links.toml");
	let external_links = fs::read_to_string(external_links_path)
		.await
		.context("Unable to read external links")?;
	let external_links = toml::from_str(&external_links).context("Unable to parse external links")?;

	Ok(Json(external_links))
}

async fn projects(extract::State(state): extract::State<Arc<State>>) -> Result<Json<Projects>, ReqError> {
	let projects_path = state.config.resources.join("projects.toml");
	let projects = fs::read_to_string(projects_path)
		.await
		.context("Unable to read projects")?;
	let projects = toml::from_str(&projects).context("Unable to parse projects")?;

	Ok(Json(projects))
}

#[derive(Debug)]
#[derive(serde::Deserialize)]
struct CvQuery {
	lang: String,
}

async fn cv(
	extract::State(state): extract::State<Arc<State>>,
	extract::Query(query): extract::Query<CvQuery>,
) -> Result<Vec<u8>, ReqError> {
	// TODO: Is this check enough? Should we instead use something like `cap_std::Dir`?
	ensure!(
		!query.lang.contains(['/', '.']),
		"Language cannot contain slashes or dots"
	);
	let cv_path = state.config.resources.join(format!("cv/{}.pdf", query.lang));

	let cv = fs::read(cv_path).await?;
	Ok(cv)
}


/// Request error
struct ReqError(AppError);

impl<E: Into<AppError>> From<E> for ReqError {
	fn from(err: E) -> Self {
		Self(err.into())
	}
}

impl IntoResponse for ReqError {
	fn into_response(self) -> axum::response::Response {
		let status = StatusCode::INTERNAL_SERVER_ERROR;
		let message = self.0.pretty().to_string();

		(status, Json(message)).into_response()
	}
}
