use std::path::Path;

use tokio::fs;

use crate::shared::archive::extract_async;

use super::{resolver::package::PackageId, discovery::core::CoreDescriptor};

pub struct Loader;

impl Loader {
  pub async fn load(&self, core: &CoreDescriptor, pkg: &PackageId) -> Result<(), anyhow::Error> {
    Ok(())
  }

  pub async fn extract(&self, core: &CoreDescriptor, pkg: &PackageId) -> Result<(), anyhow::Error> {
    extract_async(&pkg, &core.workdir).await?;

    Ok(())
  }


  pub fn is_script<P: AsRef<Path>>(&self, path: P) -> bool {
    matches!(
      path.as_ref().extension().map(|v| v.to_str()).flatten(),
      Some("cmd" | "wcs")
    )
  }

  pub async fn clean_scripts(&self, core: &CoreDescriptor) -> Result<(), anyhow::Error> {
    for i in core.workdir.read_dir()? {
      if let Ok(i) = i {
        if i.metadata()?.is_file() && self.is_script(i.path()) {

        }
      }
    }

    Ok(())
  }
}