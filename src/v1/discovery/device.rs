use std::path::PathBuf;

use serde::{Serialize, Deserialize};
use sysinfo::{System, SystemExt, DiskExt};
use dunce::canonicalize;
use std::fs;

pub fn get_disk_descriptors<'a>(sys: &'a mut System) -> Vec<DiskDescriptor> {
  sys.refresh_disks_list();
  sys.refresh_disks();

  sys.disks().iter().map(|v| DiskDescriptor {
    removable: v.is_removable(),
    name: v.name().to_os_string().to_string_lossy().to_string(),
    fs: String::from_utf8_lossy(v.file_system()).to_string(),
    mountpoint: v.mount_point().to_path_buf(),
  }).collect::<Vec<_>>()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskDescriptor {
  pub removable: bool,
  pub name: String,
  pub fs: String,
  pub mountpoint: PathBuf,
}

pub fn scan_profile(d: DiskDescriptor) -> Result<Option<ProfileRawDescriptor>, anyhow::Error> {
  let root = d.mountpoint.canonicalize()?.join("Edgeless");
  if root.try_exists()? && root.is_dir() {
    let pkgs = root.join("Resource");

    if pkgs.try_exists()? && pkgs.is_dir() {
      let version = fs::read_to_string(root.join("version.txt"))?;

      return Ok(Some(ProfileRawDescriptor { disk: d, root: canonicalize(root)?, pkgs: canonicalize(pkgs)?, version }))
    }
  }

  Ok(None)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileRawDescriptor {
  pub disk: DiskDescriptor,
  pub root: PathBuf,
  pub pkgs: PathBuf,
  pub version: String,
}
