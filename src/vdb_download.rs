

// This module contains all the functionality to download vdbs based on user input

pub async fn do_everything() {

    // Get a valid pdbid
    let pdbid = get::valid_pdbid().await;
    match web::download(&pdbid).await {
        Ok(_) => {}
        Err(e) => println!("Error: {}", e),
    }
}

// TODO:
//
//  Check if the file is already downloaded
//  Store default download directory
//  Serialize data
//


mod get {
// A module to get different levels of user input

    use std::io::{stdin};
    use super::web::DownloadError;

    // Receive and trim user input
    fn user_input() -> String {

        let mut user_input = String::new();
        stdin().read_line(&mut user_input).expect("Unable to read line");

        String::from(user_input.trim())
    }

        // Get the user input and validate it. Returns a valid 4-character alphanumeric string
    fn valid_input() -> String {

        // Greet the user
        println!("\nPlease enter the pdbid of the virus you would like to download");

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
                    println!("\nPlease enter a 4 character alphanumeric pdbid. Examples are 2ms2 or 1a34");
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

                    match msg {

                        DownloadError::Reqwest(e) => println!("Error with reqwest: {}", e),
                        DownloadError::Io(e) => println!("Error with Io: {}", e),
                        DownloadError::ViperDB(e) => println!("error with viperDB: {}", e),

                    }

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

    // Validate that the pased html is the page of an existing virus.
    pub fn viper_request(html: String) -> Result<(), String> {

        let document = Html::parse_document(html.as_str());
        let h2_selector = Selector::parse("div h2").unwrap();
        let h2_selection = document.select(&h2_selector);

        match h2_selection.count() {
            0 => {
                return Ok(());
            }
            1 => {
                return Err(String::from("No pdbid found (There exists one 'div h2')"));
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
    // use tempfile::Builder;
    use std::fs::File;
    use std::fs;
    use std::io::copy;
    use std::error::Error;
    use reqwest;
    use libflate::gzip::Decoder;

    type Result<T> = std::result::Result<T, DownloadError>;


    #[derive(Debug)]
    pub enum DownloadError {

        Reqwest(reqwest::Error),
        Io(std::io::Error),
        ViperDB(String),
        // File(String),
    }

    impl fmt::Display for DownloadError {

        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

            match &*self {
                DownloadError::Reqwest(e) => write!(f, "Reqwest::Error - {}", e),
                DownloadError::Io(e) => write!(f, "Io::Error - {}", e),
                DownloadError::ViperDB(s) => write!(f, "ViperDB::Error - {}", s),
                // DownloadError::File(s) => write!(f, "File::Error - {}", s),
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

    impl From<String> for DownloadError {
        fn from(err: String) -> DownloadError {
            DownloadError::ViperDB(err)
        }
    }

    pub async fn request_search(pdbid: &String) -> Result<String> {

        println!("--- Requestiong pdbid: {} from the ViperDB", pdbid);

        let html = match check_viperdb(pdbid).await {

            Ok(html) => html,
            Err(e) => {
                return Err(e);
            }

        };

        match super::validate::viper_request(html) {
            Ok(()) => Ok(pdbid.to_string()),
            Err(s) => Err(DownloadError::ViperDB(s)),
        }


    }

    async fn check_viperdb(pdbid: &String) -> Result<String> {

        println!("--- Entering 'check_viperd'");
        let response = reqwest::get(format!("http://viperdb.scripps.edu/SearchVirus.php?search={}&option=VDB", pdbid)).await?;
        println!("--- Running reqwest");
        Ok(response.text().await?)

    }

    pub async fn download(pdbid: &String) -> Result<()> {

        println!("--- Staring {} download", pdbid);

        let pdb_download_link = String::from(format!("http://viperdb.scripps.edu/resources/VDB/{}.vdb.gz", pdbid));
        let response = reqwest::get(pdb_download_link).await?;
        let bytes = response.bytes().await?;

        let download_folder = format!("/home/ejovo/Downloads/pdbs/{}", pdbid);
        let download_path = String::clone(&download_folder) + format!("/{}.vdb.gz", pdbid).as_str();
        let decompressed_vdb_str = download_path.as_str().replace(".gz", "");

        println!("--- Download folder: {}", download_folder);
        println!("--- Compressed file path: {}", download_path);

        fs::create_dir_all(&download_folder)?;

        println!("--- Successfuly created directories");

        let mut gz_file = File::create(&download_path)?;
        copy(&mut bytes.as_ref(), &mut gz_file)?;

        println!("--- Downloaded compressed file");

        let mut decompressed_vdb = File::create(&decompressed_vdb_str)?;
        let gz_file = File::open(&download_path)?; // Mask old file so that it goes out of scope, effectively closing the file!

        println!("--- Decompressing file");

        let mut decoder = Decoder::new(gz_file).unwrap();
        copy(&mut decoder, &mut decompressed_vdb)?;

        println!("--- File unzipped and moved to {}", decompressed_vdb_str);

        // Delete the compressed file version


        Ok(())

    }
}
