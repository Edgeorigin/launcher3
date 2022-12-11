use serde::{Serialize, Deserialize};
use sysinfo::{System, SystemExt};

use crate::v1::resolver::package::PackageId;

use super::device::{ProfileRawDescriptor, get_disk_descriptors, scan_profile};
pub use super::device::DiskDescriptor;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
  descriptor: ProfileRawDescriptor
}

impl Profile {
  pub fn fetch_disks() -> Vec<DiskDescriptor> {
    let mut sys = System::new();
    get_disk_descriptors(&mut sys)
  }

  pub fn parse_all() -> Vec<Profile> {
    Self::fetch_disks().iter().filter_map(
      |v| Self::parse(v.to_owned())
    ).collect()
  }

  pub fn parse(root: DiskDescriptor) -> Option<Profile> {
    scan_profile(root).map(
      |v| v.map(
        |descriptor| Profile { descriptor }
      )
    ).unwrap_or(None)
  }
}

impl Profile {
  pub fn scan_packages(&self, prefix: Option<&str>) -> Result<Vec<PackageId>, anyhow::Error> {
    let mut p = vec![];
    for i in self.descriptor.pkgs.read_dir()? {
      match i {
        Ok(i) if i.metadata()?.is_file() => {
          if let Ok(i) = PackageId::parse(i.path(), prefix) {
            p.push(i)
          }
        },
        _ => {}
      }
    }
    Ok(p)
  }
}