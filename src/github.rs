use rocket::serde::{Deserialize, json::serde_json};
use crate::{update::{Update, Platforms, Platform}, CACHED_UPDATE};

/// Get the latest update from the Github API.
pub(crate) fn get_release() {
    let client = reqwest::blocking::Client::new();
    let release: GithubRelease = serde_json::from_str(
        &client.get("http://api.github.com/repos/youtile/youtile/releases/latest")
        .header("User-Agent", "emberry@paperboy-0.1.0")
        .send().expect("failed to fetch latest release")
        .text().expect("failed to parse response to text")
    ).expect("failed to parse text to json");

    let info = Update {
        version: &release.tag_name,
        notes: &release.name,
        pub_date: &release.published_at,
        platforms: Platforms { 
            darwin_x86_64 : None,
            darwin_aarch64: None,
            linux_x86_64  : Some(Platform::new("", "")), 
            windows_x86_64: Some(Platform::new("", "")), 
        }
    };

    {
        let mut cache = CACHED_UPDATE.lock().expect("failed to lock json cache");
        *cache = serde_json::to_string(&info).expect("failed to parse json");
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct GithubAsset {
    name: String,
    label: String,
    browser_download_url: String
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct GithubRelease {
    tag_name: String,
    name: String,
    published_at: String,
    assets: Vec<GithubAsset>
}