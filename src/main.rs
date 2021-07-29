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
    use scraper::{Html, Selector};
    // use serde::Deserialize;

    pub async fn download() -> Result<(), ()> {

        // Get a valid pdbid
        let pdbid = get_valid_input();

        // Check to see if the pdbid is registered on viperDB
        match request_search(&pdbid).await {
            Ok(_) => println!("Success"),
            Err(e) => println!("Failure: {}", e)
        }

        println!("Downloading pdb {}", pdbid);




        Ok(())

    }

    // Receive and trim user input
    fn get_user_input() -> String {

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
                    return Err(String::from("Pdbid must only contain alphanumeric character"));
                }
            }

            return Ok(pdbid);

        } else {
            return Err(String::from("Pdbid must be four characters"));
        }
    }

    // Get the user input and validate it. Returns a valid 4-character alphanumeric string
    fn get_valid_input() -> String {

        // Greet the user
        println!("Please enter the pdbid of the virus you would like to download");

        // Get input
        let mut valid_input: String;

        loop {

            // If the input is valid, assign it to valid_input and return. Else, Restart the loop.
            match validate_pdbid_input(get_user_input()) {
                Ok(input) => {
                    valid_input = input;
                    break;
                }
                Err(msg) => {
                    println!("Error: {}", msg);
                    println!("Please enter a 4 character alphanumeric pdbid. Examples are 2ms2 or 1a34");
                    continue;
                }
            }
        }

        valid_input

    }

    // #[derive(Deserialize)]
    // struct Ip {
    //     origin: String,
    // }

    async fn request_search(pdbid: &String) -> Result<String, String> {

        println!("Requestiong pdbid: {} from the ViperDB", pdbid);

        let html = match check_viperdb(pdbid).await {

            Ok(html) => html,
            Err(e) => {
                return Err(format!("Reqwest error: {}", e));
            }

        };

        let document = Html::parse_document(html.as_str());
        let h2_selector = Selector::parse("div h2").unwrap();
        let h2_selection = document.select(&h2_selector);

        match h2_selection.count() {
            0 => {
                return Ok(String::from(pdbid));
            }
            1 => {
                return Err(String::from("No pdbid found (1 div h2)"));
            }
            _ => {
                return Err(String::from("Count of selector 'div h2' is neither 1 nor 0"));
            }
        }

    async fn check_viperdb(pdbid: &String) -> Result<String, reqwest::Error> {

        let response = reqwest::get(format!("http://viperdb.scripps.edu/SearchVirus.php?search={}&option=VDB", pdbid)).await?;
        Ok(response.text().await?)

    }






        Ok(String::from("Success"))

    }

    // This function gets a valid pdbid from the user that appears on the viperDB and returns a 4-character String
    async fn get_valid_pdbid() -> String {




        String::from("Default")
    }



}