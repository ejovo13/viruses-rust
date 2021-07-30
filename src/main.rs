extern crate async_std;

mod vdb_download;
mod cli;

// use error_chain::error_chain;
use vdb_download as vdb;




#[tokio::main]
async fn main() -> Result<(), ()> {

    cli::run();
    vdb::do_everything().await;

    Ok(())
}

