pub mod v1;

use crate::v1::resolver::file::FileId;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let id = r#".\VLC_3.0.17.4_Cno（bot）.7z"#.parse::<FileId>()?;

    println!("{:#?}", id);

    Ok(())
}
