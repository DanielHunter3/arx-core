use std::collections::HashMap;

use semver::VersionReq;
use serde::Deserialize;

use crate::component::policy::UpdatePolicy;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Dependencies {
    Simple(VersionReq),
    Detailed(DepDetails)
}

pub type DepName = String;
pub type DepHashMap = HashMap<DepName, Dependencies>;

#[derive(Debug, Deserialize)]
pub struct DepDetails {
    pub version: VersionReq,
    pub policy: UpdatePolicy,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub git: Option<String>,
    #[serde(default)]
    pub branch: Option<String>,
    #[serde(default)]
    pub features: Vec<String>,
    #[serde(default)]
    pub optional: bool,
    #[serde(default)]  // Полезно для указания конкретной версии из git
    pub rev: Option<String>,
    #[serde(default)]  // Для указания тега
    pub tag: Option<String>,
}