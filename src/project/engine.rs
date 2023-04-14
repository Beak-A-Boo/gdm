use core::fmt;
use std::str::FromStr;

use serde::Serializer;

#[derive(Debug, serde_with::DeserializeFromStr)]
pub struct EngineVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,

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
                    "{}.{}.{}-{}",
                    self.major, self.minor, self.patch, build_string
                )
            }
            None => write!(f, "{}.{}.{}", self.major, self.minor, self.patch),
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
        let mut version = version.splitn(3, '.');

        let major = version.next().and_then(|n| u8::from_str(n).ok()).unwrap();
        let minor = version.next().and_then(|n| u8::from_str(n).ok()).unwrap();
        let patch = version.next().and_then(|n| u8::from_str(n).ok()).unwrap();

        Self {
            major,
            minor,
            patch,
            build_string,
        }
    }
}
