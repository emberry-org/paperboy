use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct Platform<'a> {
    signature: &'a str,
    url: &'a str,
}

impl<'a> Platform<'a> {
    pub fn new(signature: &'a str, url: &'a str) -> Self {
        Self {
            signature, url
        }
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct Platforms<'a> {
    #[serde(rename = "darwin-x86_64")]
    pub darwin_x86_64: Option<Platform<'a>>,
    #[serde(rename = "darwin-aarch64")]
    pub darwin_aarch64: Option<Platform<'a>>,
    #[serde(rename = "linux-x86_64")]
    pub linux_x86_64: Option<Platform<'a>>,
    #[serde(rename = "windows-x86_64")]
    pub windows_x86_64: Option<Platform<'a>>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct Update<'a> {
    pub version: &'a str,
    pub notes: &'a str,
    pub pub_date: &'a str,
    pub platforms: Platforms<'a>
}