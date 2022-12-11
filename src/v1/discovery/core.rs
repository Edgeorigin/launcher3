use std::{env::var, path::PathBuf, fs::read_to_string};

use anyhow::bail;

pub struct CoreDescriptor {
  pub root: PathBuf,
  pub workdir: PathBuf,
  pub version: String
}

impl CoreDescriptor {
  pub fn new() -> Result<Self, anyhow::Error> {
    let pf = PathBuf::from(var("PROGRAMFILES")?);
    let version = read_to_string(pf.join("version.txt"))?;
    let el = pf.join("Edgeless");
    if !el.try_exists()? || !el.is_dir() {
      bail!("not found edgeless folder")
    }

    Ok(CoreDescriptor { root: pf, workdir: el, version })
  }
}
