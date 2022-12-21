pub mod v1;
pub mod shared;

use crate::v1::discovery::core::CoreDescriptor;
use crate::v1::discovery::profile::Profile;
use crate::v1::loader::Loader;
use crate::v1::discovery::device::get_disk_descriptors;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    println!("Disks: {:#?}", get_disk_descriptors()?);

    let profiles = Profile::parse_all()?;
    let core = CoreDescriptor::new()?;

    for i in profiles {
        println!("Load Profile {:?}", i);
        let pkgs = i.scan_packages(None)?;
        for j in pkgs {
            if j.flags().is_empty() {
                let l = Loader::new(&core, &j);
                if let Err(e) = l.load().await {
                    eprintln!("Error {:#?}", e)
                }
            }
        }
    }

    Ok(())
}
