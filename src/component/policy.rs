use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum RollingPolicy {
    Major,
    Minor,
    Patch,
}

#[derive(Debug, Deserialize)]
pub enum FixedPolicy {
    LTS,
    ESR,
    Frozen,
}

#[derive(Debug, Deserialize)]
pub enum UpdatePolicy {
    Fixed(FixedPolicy),
    Rolling(RollingPolicy)
} 