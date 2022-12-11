use serde::{Deserialize, Serialize};

use super::underline::{Underline, UnderlineParseError};
use std::collections::HashSet;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub const DEFAULT_EXT: &'static str = "7z";

#[derive(Debug, Clone, thiserror::Error)]
pub enum PackageIdParseError {
    #[error("path is not file")]
    NotFile,

    #[error("invalid extension")]
    InvalidExt,

    #[error("invalid file name")]
    InvalidFilename(#[from] UnderlineParseError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageId {
    path: PathBuf,
    id: Underline,
    flags: HashSet<char>,
    ext: String,
}

impl FromStr for PackageId {
    type Err = PackageIdParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s, None)
    }
}

impl From<&Path> for PackageId {
    fn from(p: &Path) -> Self {
        Self::parse(p, None).unwrap()
    }
}

impl From<PathBuf> for PackageId {
    fn from(p: PathBuf) -> Self {
        Self::parse(p, None).unwrap()
    }
}

impl From<String> for PackageId {
    fn from(p: String) -> Self {
        Self::parse(p, None).unwrap()
    }
}

impl PackageId {
    pub fn parse<P: AsRef<Path>>(p: P, e: Option<&str>) -> Result<Self, PackageIdParseError> {
        let p = p.as_ref().to_path_buf();
        let e = e.unwrap_or(DEFAULT_EXT);

        let u = p
            .file_stem()
            .ok_or(PackageIdParseError::NotFile)?
            .to_string_lossy()
            .parse::<Underline>()
            .map_err(PackageIdParseError::InvalidFilename)?;

        let f = p
            .extension()
            .ok_or(PackageIdParseError::InvalidExt)?
            .to_string_lossy()
            .strip_prefix(e)
            .ok_or(PackageIdParseError::InvalidExt)?
            .chars()
            .collect::<HashSet<_>>();

        Ok(Self {
            path: p,
            id: u,
            flags: f,
            ext: e.to_string(),
        })
    }
}

impl Deref for PackageId {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl AsRef<Path> for PackageId {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}

impl PackageId {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn id(&self) -> &Underline {
        &self.id
    }

    pub fn flags(&self) -> &HashSet<char> {
        &self.flags
    }

    pub fn extname(&self) -> &String {
        &self.ext
    }
}
