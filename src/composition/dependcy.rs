use std::collections::HashMap;

use semver::VersionReq;

use crate::component::policy::UpdatePolicy;

#[derive(Debug)]
pub enum Dependencies {
    Simple(VersionReq),
    Detailed(DepDetails)
}

pub type DepName = String;
pub type DepHashMap = HashMap<DepName, Dependencies>;

#[derive(Debug)]
pub struct DepDetails {
    pub version: VersionReq,
    pub policy: UpdatePolicy,
    pub path: Option<String>,
    pub git: Option<String>,
    pub branch: Option<String>,
    pub features: Vec<String>,
    pub optional: bool,
    pub rev: Option<String>,
    pub tag: Option<String>,
}