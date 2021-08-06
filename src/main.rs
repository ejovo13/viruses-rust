extern crate async_std;

mod cli;
mod math;
mod mol;
mod vdb;
// use error_chain::error_chain;




#[tokio::main]
async fn main() -> Result<(), ()> {

    math::print_vec();

    cli::run();
    let vdb_pair = vdb::downloader::do_everything().await;
    let vdb = vdb::reader::do_everything(vdb_pair);

    Ok(())
}

