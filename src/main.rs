pub mod v1;
pub mod shared;

use std::path::PathBuf;
use std::str::FromStr;

use crate::v1::discovery::profile::Profile;

use crate::shared::archive::extract_async;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let pkgs = Profile::parse_all().first().expect("not found profile").scan_packages(None)?;
    println!("{:#?}", pkgs);

    extract_async(PathBuf::from("./VLC_3.0.17.4_Cno（bot）.7z").canonicalize()?, PathBuf::from("./test_output")).await?;

    Ok(())
}
