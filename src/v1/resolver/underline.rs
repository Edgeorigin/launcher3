use semver::Version as Semver;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Version {
    Raw(String),
    Semver(Semver),
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Raw(s) => write!(f, "{}", s),
            Self::Semver(s) => write!(f, "{}", s),
        }
    }
}

impl FromStr for Version {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<Semver>()
            .map(Self::Semver)
            .unwrap_or_else(|_| Self::Raw(s.to_string())))
    }
}

pub trait PluginId: Clone + Serialize {
    fn name(&self) -> &String;
    fn version(&self) -> &Version;
    fn author(&self) -> &String;
    fn category(&self) -> Option<&String>;
    fn to_id(&self) -> String {
        format!("{}_{}_{}", self.name(), self.version(), self.author())
    }
    fn to_id4(&self) -> String {
        match self.category() {
            Some(c) => format!("{}_{}", self.to_id(), c),
            None => self.to_id(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Underline {
    name: String,
    version: Version,
    author: String,
    category: Option<String>,
}

impl Underline {
    pub fn uid(&self) -> String {
        blake3::hash(self.to_id().as_bytes()).to_hex().to_string()
    }
}

impl std::fmt::Display for Underline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_id())
    }
}

impl PluginId for Underline {
    fn name(&self) -> &String {
        &self.name
    }

    fn version(&self) -> &Version {
        &self.version
    }

    fn author(&self) -> &String {
        &self.author
    }

    fn category(&self) -> Option<&String> {
        self.category.as_ref()
    }
}

impl FromStr for Underline {
    type Err = UnderlineParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sp = s.split('_').map(ToString::to_string).collect::<Vec<_>>();

        if !matches!(sp.len(), 3 | 4) {
            Err(UnderlineParseError::InvalidLength(sp.len()))
        } else {
            Ok(Underline {
                name: sp[0].to_owned(),
                version: sp[1].parse().unwrap(),
                author: sp[2].to_owned(),
                category: sp.get(3).map(|v| v.to_owned()),
            })
        }
    }
}

impl From<&str> for Underline {
    fn from(s: &str) -> Self {
        Self::from_str(s).unwrap()
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum UnderlineParseError {
    #[error("invalid input, slice length = {}", .0)]
    InvalidLength(usize),
}
