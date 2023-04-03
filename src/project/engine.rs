use core::fmt;

use serde::Serializer;

#[derive(Debug)]
pub struct EngineVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,

    pub build_string: Option<String>,
}

impl fmt::Display for EngineVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(build_string) = &self.build_string {
            write!(
                f,
                "{}.{}.{}-{}",
                self.major, self.minor, self.patch, build_string
            )
        } else {
            write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
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

struct EngineVersionVisitor;

impl<'de> serde::de::Visitor<'de> for EngineVersionVisitor {
    type Value = EngineVersion;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        return formatter.write_str("a string");
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(EngineVersion::from_string(value.to_string()))
    }
}

impl<'de> serde::de::Deserialize<'de> for EngineVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
            return deserializer.deserialize_string(EngineVersionVisitor);
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

        return EngineVersion {
            major,
            minor,
            patch,
            build_string,
        };
    }
}
