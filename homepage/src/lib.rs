//! Homepage

// Imports
use url::Url;

/// ExternalLinks
#[derive(Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ExternalLinks {
	pub links: Vec<ExternalLink>,
}

/// ExternalLink
#[derive(Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ExternalLink {
	pub location: Url,
	pub text:     String,
}

/// Projects
#[derive(Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Projects {
	pub projects: Vec<Project>,
}

/// Project
#[derive(Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Project {
	pub name:        String,
	pub description: String,
	pub link:        String,
}

/// This website's source page
pub const THIS_WEBSITE: &str = "https://gitea.filipejr.com/zenithsiz/filipejr-homepage";
