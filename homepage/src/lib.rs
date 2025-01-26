//! Homepage

/// Project
#[derive(Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Project {
	pub name: String,
	pub link: String,
}

/// This website's source page
pub const THIS_WEBSITE: &str = "https://gitea.filipejr.com/zenithsiz/filipejr-homepage";
