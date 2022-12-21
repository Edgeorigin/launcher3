use std::{process::Stdio, path::Path};

use anyhow::bail;
use tokio::process::Command;

pub async fn run_wcs<P: AsRef<Path>>(path: P) -> Result<(), anyhow::Error> {
  let r = Command::new("pecmd")
    .arg("LOAD")
    .arg(path.as_ref())
    .stdin(Stdio::null())
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .spawn()?
    .wait_with_output()
    .await?;

  if r.status.success() {
    Ok(())
  } else {
    bail!("script(wcs) failed, {:#?}", r)
  }
}

pub async fn run_cmd<P: AsRef<Path>>(path: P) -> Result<(), anyhow::Error> {
  let r = Command::new("cmd")
    .arg("/c")
    .arg(path.as_ref())
    .stdin(Stdio::null())
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .spawn()?
    .wait_with_output()
    .await?;

  if r.status.success() {
    Ok(())
  } else {
    bail!("script(cmd) failed, {:#?}", r)
  }
}