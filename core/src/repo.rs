use anyhow::Result;
use serde::{Deserialize, Serialize};

const REPO_URL: &str = "https://raw.githubusercontent.com/vikpe/anyquake/main/repo";

#[derive(Serialize, Deserialize, Debug)]
pub struct Module {
    pub id: String,
    pub name: String,
    pub description: String,
    pub website: String,
    pub releases: Vec<Release>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Release {
    pub version: String,
    pub url: String,
    pub assets: Vec<ReleaseAsset>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReleaseAsset {
    pub os_arch: String,
    pub url: String,
}

pub async fn get_module_info(id: &str) -> Result<Module> {
    let url = format!("{REPO_URL}/modules/{id}.json");
    let info = reqwest::get(&url).await?.json::<Module>().await?;
    Ok(info)
}
