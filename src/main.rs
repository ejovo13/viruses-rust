extern crate async_std;

use error_chain::error_chain;
use std::io::copy;
use std::fs::File;
use tempfile::Builder;
// use async_std::main;
// use futures::executor::block_on;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio::macros;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);

    }
}

#[tokio::main]
async fn main() -> Result<()> {

    println!("Starting async_main");

    let tmp_dir = Builder::new().prefix("tmp").tempdir()?;
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let response = reqwest::get(target).await?;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("file to download {}", fname);
        let fname = tmp_dir.path().join(fname);
        println!("will be located under: '{:?}'", fname);
        File::create(fname)?
    };
    let content = response.text().await?;
    copy(&mut content.as_bytes(), &mut dest)?;

    Ok(())
}