//! Homepage

/// ProjectS
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
