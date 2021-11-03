// Read vdb files and slurp out information from them
use std::vec::Vec;
use std::fs::File;
use std::io::{self, BufRead};
use crate::virus::Virus;
use serde_json::{Value};//, Value};

// use std::path::Path;

use crate::vdb::{VDB, Atom};

pub async fn do_everything(pdbid_pair: (String, String)) -> Virus {

    let vdb = VDB::new(pdbid_pair.0.as_str(), pdbid_pair.1.as_str());
    // vdb.query_info().await;
    let info = vdb.query_backend().await;

    Virus::from(vdb, info)
}

impl VDB {

    fn new(pdbid: &str, full_path: &str) -> Self {

        let mut vdb = VDB {
            full_path: full_path.to_string(),
            num_atoms: 0,
            // vdb_read: false,
            atoms: Vec::<Atom>::new(),
            au: false,
            pdbid: pdbid.to_string()
        };

        vdb.load();
        vdb
    }

    fn new_au(pdbid: &str, full_path: &str) -> Self {

        let mut vdb = VDB {
            full_path: full_path.to_string(),
            num_atoms: 0,
            // vdb_read: false,
            atoms: Vec::<Atom>::new(),
            au: true,
            pdbid: pdbid.to_string()
        };

        vdb.load();
        vdb
    }

    // Gather info about a vdb virus using the viperdb frontend
    // Deprecated
    // async fn query_info(&self) {

    //     let body = super::downloader::web::query_info(&self.pdbid).await.unwrap();
    //     println!("Downloaded info: \n{}", body);
    // }

    async fn query_backend(&self) -> super::VDBInfo {

        let body = super::downloader::web::query_backend(&self.pdbid).await.unwrap();
        // println!("Downloaded info: \n{}", body);

        let v = serde_json::from_str(&body).unwrap();

        if let Value::Object(o) = v {

            let mut name = String::new();
            let mut family = String::new();
            let mut genus = String::new();
            let mut genome = String::new();
            let mut host = String::new();
            let mut t_number: usize = 0;
            let mut resolution: f64 = 0.;


            if let Value::String(n) = o.get("name").unwrap_or(&Value::String("N/A".to_string())) {
                name = String::from(n);
            }
            if let Value::String(fam) = o.get("family").unwrap_or(&Value::String("N/A".to_string())) {
                family = String::from(fam);
            }
            if let Value::String(gen) = o.get("genus").unwrap_or(&Value::String("N/A".to_string())) {
                genus = String::from(gen);
            }
            if let Value::String(genom) = o.get("genome").unwrap_or(&Value::String("N/A".to_string())) {
                genome = String::from(genom);
            }
            if let Value::String(ho) = o.get("host").unwrap_or(&Value::String("N/A".to_string())) {
                host = String::from(ho);
            }
            if let Value::Number(t) = o.get("tnumber").unwrap_or(&Value::String("N/A".to_string())) {
                t_number = t.as_u64().unwrap() as usize;
            }
            if let Value::Number(res) = o.get("resolution").unwrap_or(&Value::String("N/A".to_string())) {
                resolution = res.as_f64().unwrap();
            }

            println!("--- Information succesfully extracted");


            super::VDBInfo {

                name: name,
                family: family,
                genus: genus,
                genome: genome,
                host: host,
                t_number: t_number,
                resolution: resolution,
            }


        } else {
            panic!("Json object not read properly");
        }
    }


    // fn load(&mut self) -> isize {

    //     self.process_lines()
    // }

    // Interpret the lines of a vdb file and store the data in self.atoms
    //
    //@param sel
    fn load(&mut self) {

        if let Ok(lines) = self.read_lines() {

            let mut atoms = Vec::<Atom>::new();
            let mut count = 1;

            for line in lines {
                if let Ok(ip) = line {
                    // println!("{}", ip);
                    count = count + 1;
                    if let Some(atom) = Atom::from_vdb_line(ip) {
                        atoms.push(atom);
                    } else {
                        // println!("Failed to read line");
                    }
                } else {
                    println!("End of file");
                }
            }

            self.atoms = atoms;
            self.num_atoms = self.atoms.iter().count();
        }
    }

    // Open the file stored in self.full_path and return a BufReader of the file
    fn read_lines(&self) -> std::io::Result<io::Lines<io::BufReader<File>>> {

        let vdb_file = File::open(&self.full_path).unwrap();
        return Ok(io::BufReader::new(vdb_file).lines());
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn open_vdb() {

        let my_vdb = super::VDB::new("2ms2", "/home/ejovo/Downloads/pdbs/2ms2/2ms2.vdb");
        println!("Num atoms: {}", my_vdb.num_atoms);
        // println!("Max radius: {}", my_vdb.max_radius());



    }

    #[test]
    fn read_one_string() {

        let full_path = "/home/ejovo/Downloads/pdbs/2ms2/2ms2.vdb";

        let mut vdb = super::VDB {
            full_path: full_path.to_string(),
            num_atoms: 0,
            // vdb_read: false,
            atoms: Vec::<super::Atom>::new(),
            au: true,
            pdbid: "2ms2".to_string()
        };

        let lines = vdb.read_lines();
        println!("Reading lines");

        let mut done = false;
        let mut my_atom: super::Atom;

        if let Ok(lines) = lines {

            for line in lines {
                if done {
                    println!("Atom path: {}", vdb.full_path);
                    return;
                }
                if let Ok(ip) = line {
                    println!("Processing this line:\n {}", ip);
                    done = true;
                    my_atom = super::Atom::from_vdb_line(ip).expect("Atom is empty");
                }
            }
        }
    }
}


