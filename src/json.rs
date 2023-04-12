use once_cell::sync::Lazy;
use pkg_version::*;
use serde_derive::{Deserialize, Serialize};
use std::{io, sync::Mutex};

const MAJOR: u32 = pkg_version_major!();
const MINOR: u32 = pkg_version_minor!();
const PATCH: u32 = pkg_version_patch!();

/// Cache for the latest update JSON.
pub(crate) static CACHED_JSON: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("".to_string()));

/// Reload/Update the cached JSON to the latest update from Github.
pub(crate) fn reload_json() -> io::Result<String> {
    let resp = minreq::get("http://api.github.com/repos/youtile/youtile/releases/latest")
        .with_header(
            "User-Agent",
            format!("emberry@paperboy-{}.{}.{}", MAJOR, MINOR, PATCH),
        )
        .send()
        .expect("github request failed");

    let release: GithubRelease = serde_json::from_str(resp.as_str().unwrap()).unwrap();

    let info = UpdateInfo {
        version: release.tag_name,
        notes: release.name,
        pub_date: release.published_at,
        platforms: Platforms {
            darwin_x86_64: None,
            darwin_aarch64: None,
            linux_x86_64: None,
            windows_x86_64: Some(Platform {
                signature: "Content of app.msi.sig".to_owned(), // TODO: fill this
                url: release.assets.get(0).unwrap().browser_download_url.clone(),
            }),
        },
    };

    {
        let mut cache = CACHED_JSON.lock().expect("failed to lock json cache");
        *cache = serde_json::to_string(&info).expect("failed to parse json");
    }

    Ok(info.version)
}

#[derive(Deserialize, Debug)]
struct GithubAsset {
    // name: String,
    // label: String,
    browser_download_url: String,
}

#[derive(Deserialize, Debug)]
struct GithubRelease {
    tag_name: String,
    name: String,
    published_at: String,
    assets: Vec<GithubAsset>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateInfo {
    pub version: String,
    pub notes: String,
    pub pub_date: String,
    pub platforms: Platforms,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Platforms {
    #[serde(rename = "darwin-x86_64")]
    pub darwin_x86_64: Option<Platform>,
    #[serde(rename = "darwin-aarch64")]
    pub darwin_aarch64: Option<Platform>,
    #[serde(rename = "linux-x86_64")]
    pub linux_x86_64: Option<Platform>,
    #[serde(rename = "windows-x86_64")]
    pub windows_x86_64: Option<Platform>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Platform {
    pub signature: String,
    pub url: String,
}
