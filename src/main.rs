pub mod v1;

use crate::v1::resolver::file::FileId;
use sevenz_rust::{SevenZReader, default_entry_extract_fn};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let id = r#".\VLC_3.0.17.4_Cno（bot）.7z"#.parse::<FileId>()?;

    println!("{:#?}", id);

    Ok(())
}

async fn extract(id: FileId) -> Result<(), anyhow::Error> {
    let mut a = SevenZReader::open(&id, &[])?;

    let mut t = vec![];

    a.for_each_entries(move |a, b| {
        t.push(extract_one());
        Ok(true)
    });

    Ok(())
}


async fn extract_one() -> Result<(), anyhow::Error> {
    Ok(())
}