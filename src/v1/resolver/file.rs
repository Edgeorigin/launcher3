use serde::{Serialize, Deserialize};

use super::underline::{Underline, UnderlineParseError};
use std::collections::HashSet;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub const DEFAULT_EXT: &'static str = "7z";

#[derive(Debug, Clone, thiserror::Error)]
pub enum FileIdParseError {
    #[error("path is not file")]
    NotFile,

    #[error("invalid extension")]
    InvalidExt,

    #[error("invalid file name")]
    InvalidFilename(#[from] UnderlineParseError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileId {
    path: PathBuf,
    id: Underline,
    flags: HashSet<char>,
    ext: String,
}

impl FromStr for FileId {
    type Err = FileIdParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s, None)
    }
}

impl From<&Path> for FileId {
    fn from(p: &Path) -> Self {
        Self::parse(p, None).unwrap()
    }
}

impl From<PathBuf> for FileId {
    fn from(p: PathBuf) -> Self {
        Self::parse(p, None).unwrap()
    }
}

impl From<String> for FileId {
    fn from(p: String) -> Self {
        Self::parse(p, None).unwrap()
    }
}

impl FileId {
    pub fn parse<P: AsRef<Path>>(p: P, e: Option<&str>) -> Result<Self, FileIdParseError> {
        let p = p.as_ref().to_path_buf();
        let e = e.unwrap_or(DEFAULT_EXT);

        let u = p
            .file_stem()
            .ok_or(FileIdParseError::NotFile)?
            .to_string_lossy()
            .parse::<Underline>()
            .map_err(FileIdParseError::InvalidFilename)?;

        let f = p
            .extension()
            .ok_or(FileIdParseError::InvalidExt)?
            .to_string_lossy()
            .strip_prefix(e)
            .ok_or(FileIdParseError::InvalidExt)?
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

impl Deref for FileId {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl AsRef<Path> for FileId {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}

impl FileId {
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
