use std::path::Path;

use anyhow::bail;
use tokio::process::Command;
use tokio::fs;
use dunce::canonicalize;
use std::process::Stdio;

pub async fn extract_async<P: AsRef<Path>, S: AsRef<Path>>(i: P, o: S) -> Result<(), anyhow::Error> {
  fs::create_dir_all(&o).await?;

  let r = Command::new("7z")
    .current_dir(canonicalize(&o)?)
    .arg("x")
    .arg("-o.\\")
    .arg("-aos")
    .arg("-y")
    .arg(canonicalize(i)?)
    .stdin(Stdio::null())
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .spawn()?
    .wait_with_output()
    .await?;

  if r.status.success() {
    Ok(())
  } else {
    bail!("extract failed, {:#?}", r)
  }
}