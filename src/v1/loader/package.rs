use std::path::{Path, PathBuf};

use dunce::canonicalize;
use tokio::fs;

use crate::shared::archive::extract_async;

use super::script::{run_cmd, run_wcs};

use crate::v1::{resolver::{package::PackageId, PluginId}, discovery::core::CoreDescriptor, constant};

pub struct PackageLoader<'a> {
  core: &'a CoreDescriptor,
  pkg: &'a PackageId
}

impl PackageLoader<'_> {
  pub fn new<'a>(core: &'a CoreDescriptor, pkg: &'a PackageId) -> PackageLoader<'a> {
    PackageLoader { core, pkg }
  }
}

impl<'a> PackageLoader<'a> {
  pub async fn load(&self) -> Result<(), anyhow::Error> {
    println!("[{}] start load", self.pkg.id());
    println!("[{}] pre-clean", self.pkg.id());
    self.clean_scripts(Some("_unknown")).await?;
    println!("[{}] extract", self.pkg.id());
    self.extract().await?;
    println!("[{}] run scripts", self.pkg.id());
    self.run_scripts().await?;
    println!("[{}] post-clean", self.pkg.id());
    self.clean_scripts(Some(&self.pkg.id().to_id())).await?;
    Ok(())
  }

  pub async fn extract(&self) -> Result<(), anyhow::Error> {
    extract_async(&self.pkg, &self.core.workdir).await?;
    Ok(())
  }


  #[inline]
  pub fn is_script<P: AsRef<Path>>(&self, path: P) -> bool {
    matches!(
      path.as_ref().extension().map(|v| v.to_str()).flatten(),
      Some("cmd" | "wcs")
    )
  }

  pub async fn run_scripts(&self) -> Result<Vec<PathBuf>, anyhow::Error> {
    let mut scripts = vec![];
    for i in self.core.workdir.read_dir()? {
      if let Ok(i) = i {
        if i.metadata()?.is_file() && self.is_script(i.path()) {
          println!("[{}] run script {:?}", self.pkg.id(), i.path());
          match i.path().extension().map(|v| v.to_str()).flatten() {
            Some("cmd") => run_cmd(&i.path()).await?,
            Some("wcs") => run_wcs(&i.path()).await?,
            _ => { }
          }
          scripts.push(i.path())
        }
      }
    }

    Ok(scripts)
  }

  pub async fn clean_scripts(&self, subdir: Option<&str>) -> Result<Vec<PathBuf>, anyhow::Error> {
    let output = self.core.workdir.join(constant::SCRIPT_HISTORY_DIR).join(subdir.unwrap_or("."));
    fs::create_dir_all(&output).await?;
    let output = canonicalize(&output)?;
    let mut scripts = vec![];

    for i in self.core.workdir.read_dir()? {
      if let Ok(i) = i {
        if i.metadata()?.is_file() && self.is_script(i.path()) {
          println!("[{}] clean script {:?}", self.pkg.id(), i.path());
          let script = output.join(i.path().strip_prefix(&self.core.workdir)?);
          fs::rename(i.path(), &script).await?;
          scripts.push(canonicalize(&script)?);
        }
      }
    }

    Ok(scripts)
  }
}