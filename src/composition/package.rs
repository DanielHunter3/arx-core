use semver::Version;

#[derive(Debug)]  // Serialize может пригодиться
pub struct Package {
    pub name: String,
    pub version: Version,
    pub edition: Option<String>,
    pub authors: Vec<String>,
    pub description: Option<String>,
    pub license: Option<String>,
    pub publish: Option<bool>,
}