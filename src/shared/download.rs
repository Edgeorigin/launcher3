use std::path::PathBuf;
use std::io::{SeekFrom};
use anyhow::bail;
use num_cpus::get as get_os_threads;
use reqwest::{Client, Url, get};
use tokio::spawn;
use futures_util::StreamExt;
use tokio::fs;
use tokio::io::{AsyncWriteExt, AsyncSeekExt};
use anyhow::anyhow;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

pub async fn normal_download(url: Url, dest: PathBuf) -> Result<(), anyhow::Error> {
  let r = get(url).await?;
  let mut f = fs::File::create(dest).await?;
  let mut s = r.bytes_stream();

  while let Some(b) = s.next().await {
    f.write(&b?).await?;
  }

  f.flush().await?;

  Ok(())
}

pub async fn chunk_download(url: Url, dest: PathBuf, range: (u64, u64), n: usize, p0: ProgressBar) -> Result<(), anyhow::Error> {
  let r = Client::new()
    .get(url)
    .header("range", format!("bytes={}-{}", range.0, range.1))
    .send().await?;
  let mut s = r.bytes_stream();
  let mut c = range.0;

  // Shared
  let mut f = fs::OpenOptions::new()
    .write(true)
    .read(true)
    .open(&dest).await?;

  while let Some(b) = s.next().await {
    let b = b?;
    f.seek(SeekFrom::Start(c)).await?;
    f.write(&b).await?;
    let k = b.len() as u64;
    // pb.inc(k);
    p0.inc(k);
    c += k;
  }

  Ok(())
}

pub async fn download(url: Url, dest: PathBuf, mp: Option<&MultiProgress>, msg: Option<String>) -> Result<(), anyhow::Error> {
  let client = Client::new();
  let r = client.head(url.clone()).send().await?;
  if !r.status().is_success() {
    bail!("head request failed");
  }
  let h = r.headers();
  let b = h.get("accept-ranges");

  match b.as_ref() {
    Some(v) if v.to_str()? == "bytes" => {
      let total_length: u64 = h.get("content-length").ok_or(anyhow!("no content-length"))?.to_str()?.parse()?;
      println!("{:?}", total_length);
      let thread_num = {
        let n = get_os_threads() as u64;
        match total_length {
          0..=8_000_000 => 1,
          _ => n,
        }
      };
      let chunk_length = total_length / thread_num;
      let n_length = total_length - (chunk_length * thread_num);
      let mut k: u64 = 0;
      let mut j = (0..thread_num).map(|_| {
        let h = k;
        k += chunk_length;
        (h, k)
      }).collect::<Vec<_>>();
      j.last_mut().map(|v| v.1 += n_length);


      let m = MultiProgress::new();
      let sty = ProgressStyle::with_template("{msg:15} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")?
      .progress_chars("##-");

      fs::remove_file(&dest).await?;
      let mut f = fs::File::create(&dest).await?;
      f.set_len(total_length).await?;
      f.flush().await?;
      drop(f);

      let p = ProgressBar::new(total_length);
      p.set_style(sty.clone());
      if let Some(msg) = msg {
        p.set_message(msg);
      }

      if let Some(m) = mp {
        m.add(p.clone());
      }

      let mut h = vec![];
      for (k, r) in j.iter().enumerate() {
        h.push(spawn(chunk_download(url.clone(), dest.clone(), r.to_owned(), k, p.clone())));
      }

      for h in h {
        h.await??;
      }

      p.finish();
    },
    _ => return normal_download(url, dest).await
  }

  Ok(())
}


