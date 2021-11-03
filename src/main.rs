extern crate async_std;

mod cli;
mod math;
mod mol;
mod vdb;
mod virus;
// use error_chain::error_chain;




#[tokio::main]
async fn main() -> Result<(), ()> {

    // math::print_vec();

    cli::run();
    let virus = vdb::reader::do_everything(vdb::downloader::do_everything().await).await;

    println!("--- Virus downloaded and loaded:\n");
    println!("{}", virus);

    Ok(())
}

