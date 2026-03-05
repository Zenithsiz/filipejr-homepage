//! Homepage backend

// Features
#![feature(stmt_expr_attributes, proc_macro_hygiene, yeet_expr)]

// Imports
use {
	app_error::{AppError, Context, ensure},
	axum::{
		Json,
		extract::{Query, State},
		http::StatusCode,
		response::IntoResponse,
	},
	homepage::{Project, THIS_WEBSITE},
	std::{
		io,
		net::{IpAddr, Ipv4Addr, SocketAddr},
		path::{Path, PathBuf},
		sync::Arc,
	},
	tokio::fs,
};

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
struct Config {
	/// Port
	port: u16,

	/// Resources directory
	resources: PathBuf,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			port:      8081,
			resources: PathBuf::from("resources/"),
		}
	}
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
	tracing_subscriber::fmt().init();

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
	let config = Arc::new(config);

	// Then build the app
	let app = {
		use axum::routing::get;
		axum::Router::new()
			.route("/projects", get(self::projects))
			.route("/cv.pdf", get(self::cv))
			.with_state(Arc::clone(&config))
	};

	let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), config.port);
	let listener = tokio::net::TcpListener::bind(addr)
		.await
		.context("Unable to create tcp listener")?;
	axum::serve(listener, app)
		.await
		.context("Unable to start http server")?;

	Ok(())
}

async fn projects() -> Json<Vec<Project>> {
	let projects = vec![
		Project {
			name: "[ddw3] Digimon world 2003 decompilation".to_owned(),
			link: "https://gitea.filipejr.com/zenithsiz/ddw3".to_owned(),
		},
		Project {
			name: "[zbuild] Make-like build system".to_owned(),
			link: "https://gitea.filipejr.com/zenithsiz/zbuild".to_owned(),
		},
		Project {
			name: "[zsw] Zenithsiz's scrolling wallpaper".to_owned(),
			link: "https://gitea.filipejr.com/zenithsiz/zsw".to_owned(),
		},
		Project {
			name: "🚧 [dynatos] Rust web framework".to_owned(),
			link: "https://gitea.filipejr.com/zenithsiz/dynatos".to_owned(),
		},
		Project {
			name: "[filipejr-homepage] This page".to_owned(),
			link: THIS_WEBSITE.to_owned(),
		},
	];

	Json(projects)
}

#[derive(Debug)]
#[derive(serde::Deserialize)]
struct CvQuery {
	lang: String,
}

async fn cv(State(config): State<Arc<Config>>, Query(query): Query<CvQuery>) -> Result<Vec<u8>, ReqError> {
	// TODO: Is this check enough? Should we instead use something like `cap_std::Dir`?
	ensure!(
		!query.lang.contains(['/', '.']),
		"Language cannot contain slashes or dots"
	);
	let cv_path = config.resources.join(format!("cv/{}.pdf", query.lang));

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
