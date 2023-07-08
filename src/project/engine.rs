use core::fmt;
use std::str::FromStr;

use serde::Serializer;

#[derive(Debug, serde_with::DeserializeFromStr)]
pub struct EngineVersion {
    
    pub version_string: String,
    pub build_string: Option<String>,
}

impl FromStr for EngineVersion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(EngineVersion::from_string(s.to_string()))
    }
}

impl fmt::Display for EngineVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.build_string {
            Some(build_string) => {
                write!(
                    f,
                    "{}-{}",
                    self.version_string, build_string
                )
            }
            None => write!(f, "{}", self.version_string),
        }
    }
}

impl serde::ser::Serialize for EngineVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl EngineVersion {
    pub fn from_string(s: String) -> EngineVersion {
        let (version, build) = match s.split_once('-') {
            Some((version, build)) => (version, Some(build)),
            None => (s.as_str(), None),
        };
        let build_string = build.map(|s| s.to_owned());
        Self {
            version_string: version.to_string(),
            build_string,
        }
    }
}
