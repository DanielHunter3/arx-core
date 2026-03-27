use semver::Version;
use serde::Deserialize;

#[derive(Debug, Deserialize)]  // Serialize может пригодиться
pub struct Package {
    pub name: String,
    pub version: Version,
    #[serde(default)]
    pub edition: Option<String>,
    #[serde(default)]
    pub authors: Vec<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(default)]  // Может пригодиться для указания типа пакета
    pub publish: Option<bool>,
}