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
    let pdbid_tuplet = vdb::do_everything().await;

    let mut vdb = vdb_reader::VDB::new(&pdbid_tuplet.0);
    vdb.load();
    println!("{}", vdb);

    // for a in vdb.atoms.iter() {

    // }


    Ok(())
}

