use serde::Deserialize;

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

impl<'de> Deserialize<'de> for UpdatePolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> 
    {
        let s: String = Deserialize::deserialize(deserializer)?;

        match s.split("::").collect::<Vec<_>>().as_slice() {
            ["fixed", policy] => Ok(UpdatePolicy::Fixed(match *policy {
                "LTS" => FixedPolicy::LTS,
                "ESR" => FixedPolicy::ESR,
                "frozen" => FixedPolicy::Frozen,
                _ => return Err(serde::de::Error::custom(
                    format!("unknown fixed policy: {}", policy)
                )),
            })),
            ["rolling", policy] => Ok(UpdatePolicy::Rolling(match *policy {
                "major" => RollingPolicy::Major,
                "minor" => RollingPolicy::Minor,
                "patch" => RollingPolicy::Patch,
                _ => return Err(serde::de::Error::custom(
                    format!("unknown rolling policy: {}", policy)
                )),
            })),
            _ => Err(serde::de::Error::custom(
                "expected format: 'fixed::POLICY' or 'rolling::POLICY'"
            )),
        }
    }
}