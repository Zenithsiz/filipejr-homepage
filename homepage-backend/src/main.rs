//! Homepage backend

// Imports
use {
	axum::Json,
	homepage::{Project, THIS_WEBSITE},
	std::{
		io,
		net::{IpAddr, Ipv4Addr, SocketAddr},
		path::Path,
	},
	tokio::fs,
	zutil_app_error::{AppError, AppErrorContext},
};

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
struct Config {
	/// Port
	port: u16,
}

impl Default for Config {
	fn default() -> Self {
		Self { port: 8081 }
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

	// Then build the app
	let app = {
		use axum::routing::get;
		axum::Router::new().route("/projects", get(projects))
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
