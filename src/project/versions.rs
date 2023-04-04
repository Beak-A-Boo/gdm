use error_chain::error_chain;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use super::engine::EngineVersion;

static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct GithubReleaseResponse {
    pub tag_name: String,
}

pub async fn get_latest_version_from_github() -> Result<EngineVersion> {
    let url = "https://api.github.com/repos/godotengine/godot/releases/latest";

    let client = reqwest::Client::builder()
    .user_agent(APP_USER_AGENT)
    .build()?;

    let result = client.get(url).send().await?;

    if result.status().is_success() {
        let release_data = result.json::<GithubReleaseResponse>().await?;
        Ok(EngineVersion::from_string(release_data.tag_name))
    }
    else {
        Err(format!("Failed to get latest version from GitHub, received status code: {}", result.status()).into())
    }
}
