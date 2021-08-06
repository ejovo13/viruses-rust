extern crate async_std;

mod vdb_download;
mod cli;
mod math;
mod mol;
mod vdb_reader;

// use error_chain::error_chain;
use vdb_download as vdb;




#[tokio::main]
async fn main() -> Result<(), ()> {

    math::print_vec();

    cli::run();
    let vdb_pair = vdb::do_everything().await;
    let vdb = vdb_reader::do_everything(vdb_pair);


    Ok(())
}

