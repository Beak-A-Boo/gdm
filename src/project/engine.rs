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
    pub fn from_string(version: String) -> EngineVersion {
        //FIXME handle case where there is no '-' in the version string
        let parts = version.split_once('-').unwrap();

        let split: Vec<&str> = parts.0.splitn(3, ".").collect();

        let major = split.get(0).unwrap().parse::<u8>().unwrap();
        let minor = split.get(1).unwrap().parse::<u8>().unwrap();
        let patch = split.get(2).unwrap().parse::<u8>().unwrap();

        let build_string = Some(parts.1.to_string());

        EngineVersion {
            major,
            minor,
            patch,
            build_string,
        }
    }
}
