extern crate async_std;

use error_chain::error_chain;
use std::io::copy;
use std::fs::File;
use tempfile::Builder;
use vdb_download as vdb;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);

    }
}

#[tokio::main]
async fn main() -> Result<()> {

    match vdb::download().await {
        Ok(_) => println!("Download succesful!"),
        Err(string) => println!("{:?}", string),
    }



    // let tmp_dir = Builder::new().prefix("tmp").tempdir()?;
    // let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    // let response = reqwest::get(target).await?;

    // let mut dest = {
    //     let fname = response
    //         .url()
    //         .path_segments()
    //         .and_then(|segments| segments.last())
    //         .and_then(|name| if name.is_empty() { None } else { Some(name) })
    //         .unwrap_or("tmp.bin");

    //     println!("file to download {}", fname);
    //     let fname = tmp_dir.path().join(fname);
    //     println!("will be located under: '{:?}'", fname);
    //     File::create(fname)?
    // };
    // let content = response.text().await?;
    // copy(&mut content.as_bytes(), &mut dest)?;

    Ok(())
}

mod vdb_download {

    // This module contains all the functionality to download vdbs based on user input

    use std::io::{stdin};
    use reqwest::{Client, StatusCode};

    pub async fn download() -> Result<(), ()> {

        let pdbid: String;

        loop {
            // If the input is not valid, loop around again.
            match validate_pdbid_input(get_user_input()) {
                Ok(validated_input) => {
                    pdbid = validated_input;
                    break;
                }
                Err(_) => {
                    continue;
                }
            }
        };

        // Check to see if the pdbid is registered on viperDB
        match request_search(&pdbid).await {
            Ok(_) => println!("Success"),
            Err(_) => println!("Faulure")
        }


        println!("Downloading pdb {}", pdbid);




        Ok(())

    }

    fn get_user_input() -> String {

        // Greet the user
        println!("Please enter the pdbid of the virus you would like to download");

        let mut user_input = String::new();
        stdin().read_line(&mut user_input).expect("Unable to read line");

        String::from(user_input.trim())
    }

    fn validate_pdbid_input(pdbid: String) -> Result<String, String> {

        // println!("Validing user input: {}", pdbid);

        if pdbid.chars().count() == 4 {

            for c in pdbid.chars() {
                if c.is_alphanumeric() {
                    continue;
                } else {
                    println!("Character: '{}' is not alphanumeric", c);
                    return Err(String::from("Pdbid must only contain alphanumeric character"));
                }
            }

            return Ok(pdbid);

        } else {

            println!("Pdbid must be four characters");
            return Err(String::from("Pdbid must be four characters"));
        }
    }

    async fn request_search(pdbid: &String) -> Result<String, reqwest::Error> {

        println!("Requestiong pdbid: {} from the ViperDB", pdbid);

        let response = reqwest::get(format!("http://viperdb.scripps.edu/SearchVirus.php?search={}&option=VDB", pdbid)).await?;

        match response.status() {
            StatusCode::OK => println!("success!"),
            s => println!("Received response status: {:?}", s),
        };

        Ok(String::from("Success"))

    }



}