extern crate async_std;

// use error_chain::error_chain;
use vdb_download as vdb;


#[tokio::main]
async fn main() -> Result<(), ()> {

    vdb::do_everything();

    Ok(())
}

mod vdb_download {

    // This module contains all the functionality to download vdbs based on user input

    pub async fn do_everything() {

        // Get a valid pdbid
        let pdbid = get::valid_pdbid().await;
        web::download(pdbid);

    }


    mod get {
    // A module to get different levels of user input

        use std::io::{stdin};

        // Receive and trim user input
        fn user_input() -> String {

            let mut user_input = String::new();
            stdin().read_line(&mut user_input).expect("Unable to read line");

            String::from(user_input.trim())
        }

            // Get the user input and validate it. Returns a valid 4-character alphanumeric string
        fn valid_input() -> String {

            // Greet the user
            println!("Please enter the pdbid of the virus you would like to download");

            // Get input
            let valid_input: String;

            loop {

                // If the input is valid, assign it to valid_input and return. Else, Restart the loop.
                match super::validate::pdbid_input(user_input()) {
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

        // This function gets a valid pdbid from the user that appears on the viperDB and returns a 4-character String
        pub async fn valid_pdbid() -> String {

            let valid_pdbid: String;

            loop {

                match super::web::request_search(&valid_input()).await {
                    Ok(existing_pdbid) => {
                        valid_pdbid = existing_pdbid;
                        break;
                    }
                    Err(msg) => {
                        println!("Error: {}", msg);
                        continue;
                    }
                }
            }

            valid_pdbid
        }
    }

    mod validate {
    // A module to validate user input

        use scraper::{Html, Selector};

        pub fn pdbid_input(pdbid: String) -> Result<String, String> {

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

        pub fn viper_request(html: String) -> Result<(), String> {

            let document = Html::parse_document(html.as_str());
            let h2_selector = Selector::parse("div h2").unwrap();
            let h2_selection = document.select(&h2_selector);

            match h2_selection.count() {
                0 => {
                    return Ok(());
                }
                1 => {
                    return Err(String::from("No pdbid found (1 div h2)"));
                }
                _ => {
                    return Err(String::from("Count of selector 'div h2' is neither 1 nor 0"));
                }
            }
        }

    }

    mod web {
    // A module to access the web to download pdbs.

        use std::fmt;
        use tempfile::Builder;
        use std::fs::File;
        use std::io::copy;
        use std::error::Error;

        type Result<T> = std::result::Result<T, DownloadError>;


        #[derive(Debug)]
        enum DownloadError {

            Reqwest(reqwest::Error),
            Io(std::io::Error),
            // File(std::fs::Error),

        }

        impl fmt::Display for DownloadError {

            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    DownloadError::Reqwest(e) => write!(f, format!("{}", e)),
                    DownloadError::Io(e) => write!(f, format!("{}", e)),
                    // DownlaodError::File(e) => write!(f, format!("{}", e)),
                }
            }

        }

        impl Error for DownloadError {

        }

        impl From<std::io::Error> for DownloadError {
            fn from(err: std::io::Error) -> DownloadError {
                DownloadError::Io(err)
            }
        }

        impl From<reqwest::Error> for DownloadError {
            fn from(err: reqwest::Error) -> DownloadError {
                DownloadError::Reqwest(err)
            }
        }

        pub async fn request_search(pdbid: &String) -> Result<String> {

            println!("Requestiong pdbid: {} from the ViperDB", pdbid);

            let html = match check_viperdb(pdbid).await {

                Ok(html) => html,
                Err(e) => {
                    return Err(format!("Reqwest error: {}", e));
                }

            };

            match super::validate::viper_request(html) {

            }


        }

        async fn check_viperdb(pdbid: &String) -> Result<String> {

            let response = reqwest::get(format!("http://viperdb.scripps.edu/SearchVirus.php?search={}&option=VDB", pdbid)).await?;
            Ok(response.text().await?)

        }

        pub async fn download(pdbid: String) -> Result<()> {

            println!("Downloading pdb {}", pdbid);

            let pdb_download_link = String::from(format!("http://viperdb.scripps.edu/resources/VDB/{}.vdb.gz)", pdbid));

            let tmp_dir = Builder::new().prefix(&pdbid).tempdir().unwrap();
            let target = pdb_download_link;
            let response = reqwest::get(target).await.unwrap();

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
                File::create(fname).unwrap()
            };
            let content = response.text().await.expect("Oh no");
            copy(&mut content.as_bytes(), &mut dest)?;


            Ok(())

        }


    }









}