#[derive(Debug, PartialEq)]
pub enum RollingPolicy {
    Major,  
    Minor,
    Patch,
}

#[derive(Debug, PartialEq)]
pub enum FixedPolicy {
    LTS,
    ESR,
    Frozen,
}

#[derive(Debug, PartialEq)]
pub enum UpdatePolicy {
    Fixed(FixedPolicy),
    Rolling(RollingPolicy)
}