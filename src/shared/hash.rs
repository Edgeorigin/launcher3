use std::{io, path::Path};
use blake3::Hash;
use tokio::{fs::File, io::AsyncReadExt};


pub async fn try_into_memmap_file(file: &File) -> anyhow::Result<Option<io::Cursor<memmap2::Mmap>>> {
  let metadata = file.metadata().await?;
  let file_size = metadata.len();

  Ok(if !metadata.is_file() {
      None
  } else if file_size > isize::max_value() as u64 {
      None
  } else if file_size == 0 {
      None
  } else if file_size < 16 * 1024 {
      None
  } else {
      let mmap = unsafe {
          memmap2::MmapOptions::new()
              .len(file_size as usize)
              .map(file)?
      };

      Some(io::Cursor::new(mmap))
  })
}

async fn copy_wide(mut reader: impl tokio::io::AsyncRead + Unpin, hasher: &mut blake3::Hasher) -> io::Result<u64> {
  let mut buffer = [0; 65536];
  let mut total = 0;
  loop {
      match reader.read(&mut buffer).await {
          Ok(0) => return Ok(total),
          Ok(n) => {
              hasher.update(&buffer[..n]);
              total += n as u64;
          }
          Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
          Err(e) => return Err(e),
      }
  }
}

pub async fn compute_hash_blake3<P: AsRef<Path>>(path: P) -> anyhow::Result<Hash> {
  let file = File::open(&path).await?;
  let mut hasher = blake3::Hasher::new();
  if let Some(mmap) = try_into_memmap_file(&file).await? {
      hasher.update_rayon(mmap.get_ref());
  } else {
      copy_wide(file, &mut hasher).await?;
  }

  let hash = hasher.finalize();

  Ok(hash)
}