use std::path::Path;

use tokio::fs;

use crate::shared::archive::extract_async;

use super::{resolver::package::PackageId, discovery::core::CoreDescriptor};

pub struct Loader<'a> {
  core: &'a CoreDescriptor,
  pkg: &'a PackageId
}

impl<'a> Loader<'a> {
  pub async fn load(&self) -> Result<(), anyhow::Error> {
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

  pub async fn clean_scripts(&self) -> Result<(), anyhow::Error> {
    for i in self.core.workdir.read_dir()? {
      if let Ok(i) = i {
        if i.metadata()?.is_file() && self.is_script(i.path()) {

        }
      }
    }

    Ok(())
  }
}