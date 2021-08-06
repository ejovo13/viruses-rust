// Read vdb files and slurp out information from them
use std::str::FromStr;
use std::vec::Vec;
use std::fs::File;
use std::io::{self, BufRead};
// use std::path::Path;

struct VDB {

    full_path: String,
    num_atoms: usize,
    // vdb_read: bool, // Determine whether the vdb has actually been read
    atoms: Vec<Atom>,
    au: bool, // If true, only load the coordinates of  the AU
    pdbid: String,

}

// This structure represents all of the information that can be extracted from a vdb webpage
struct VDBInfo {

    name: String,
    pdbid: String,
    family: String,
    genus: String,
    genome: String,
    host: String,
    t_number: u64,
    n_atoms: u64,
    resolution: f64,

}

pub fn do_everything(pdbid_pair: (String, String)) {

    let vdb = VDB::new(pdbid_pair.0.as_str(), pdbid_pair.1.as_str());




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
    fn query_info() {






    }

    fn query_backend() {

    }


    // fn load(&mut self) -> isize {

    //     self.process_lines()
    // }

    // Interpret the lines of a vdb file and store the data in self.atoms
    //
    //@param sel
    fn load(&mut self) -> isize {

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

            count
        } else {
            0
        }
    }

    // Open the file stored in self.full_path and return a BufReader of the file
    fn read_lines(&self) -> std::io::Result<io::Lines<io::BufReader<File>>> {

        let vdb_file = File::open(&self.full_path).unwrap();
        return Ok(io::BufReader::new(vdb_file).lines());
    }

    fn max_radius(&self) -> f64 {

        let mut max_r = 0.;
        for a in self.atoms.iter() {
            if a.radius() > max_r {
                max_r = a.radius();
            }
        }

        max_r
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn open_vdb() {

        let my_vdb = super::VDB::new("2ms2", "/home/ejovo/Downloads/pdbs/2ms2/2ms2.vdb");
        println!("Num atoms: {}", my_vdb.num_atoms);
        println!("Max radius: {}", my_vdb.max_radius());



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














struct Atom {

    serial_number: u64, // 7-11
    atom_name: String, // 13-16
    alternate_location_indicator: char, // 17
    residue_name: String,
    chain_identifier: char,
    residue_sequence_number: u64,
    insertion_code: u64,
    x: f64,
    y: f64,
    z: f64,
    occupancy: String,
    temperature_factor: String,
    // element_symbol: String,
}

impl Atom {

    fn new(serial_number: u64, atom_name: String, alternate_location_indicator: char,
            residue_name: String, chain_identifier: char, residue_sequence_number: u64,
            insertion_code: u64, x: f64, y: f64, z: f64, occupancy: String, temperature_factor: String) -> Self {
            //element_symbol: String) -> Self {

        Atom {

            serial_number,
            atom_name,
            alternate_location_indicator,
            residue_name,
            chain_identifier,
            residue_sequence_number,
            insertion_code,
            x,
            y,
            z,
            occupancy,
            temperature_factor,
            // element_symbol,

        }
    }

    fn x(&self) -> f64 {
        return self.x;
    }

    fn y(&self) -> f64 {
        return self.y;
    }

    fn z(&self) -> f64 {
        return self.z;
    }


    fn from_vdb_line(line: String) -> Option<Atom> {

        let header = &line[0..7];

        // println!("entered vdb_line");
        // println!("\n Header recorded ad: {}", header);

        if !header.contains("ATOM") {
            return None;
        }

        let serial_number: u64 = (&line[6..11]).parse::<u64>().unwrap_or(9999999);
        // println!("serial_number:   {}", serial_number);
        let atom_name: String = (&line[12..16]).to_string();
        // println!("atom_name:   {}", atom_name);
        let alternate_location_indicator: char = line.chars().nth(16).unwrap_or(' ');
        // println!("alternate_location_indicator:   {}", alternate_location_indicator);
        let residue_name: String = (&line[17..20]).to_string();
        // println!("residue_name:   {}", residue_name);
        let chain_identifier: char = line.chars().nth(21).unwrap_or('z');
        // println!("chain_identifier:   {}", chain_identifier);
        let residue_sequence_number: u64 = (&line[22..26]).parse::<u64>().unwrap_or(0);
        // println!("res seq num:   {}", residue_sequence_number);
        let insertion_code: u64 = (&line[26..27]).parse::<u64>().unwrap_or(0);
        // println!("insertion code:   {}", insertion_code);
        // println!("X slice is converting: {}", &line[30..38]);
        let x: f64 = f64::from_str(&line[30..38].trim()).expect("Something broke at x");
        // println!("x:   {}", x);
        let y: f64 = (&line[38..46].trim()).parse::<f64>().unwrap();
        // println!("y:   {}", y);
        let z: f64 = (&line[46..54].trim()).parse::<f64>().unwrap();
        // println!("z:   {}", z);
        let occupancy: String = (&line[54..60]).to_string();
        // println!("occ:   {}", occupancy);
        let temperature_factor: String = (&line[60..66]).to_string();
        // println!("temp:   {}", temperature_factor);
        // The element_symbol is out of bounds for most vdbs i guess
        // let element_symbol: String = (&line[76..78]).to_string();
        // println!("elem:   {}", element_symbol);

        Some(Atom::new(serial_number, atom_name, alternate_location_indicator, residue_name, chain_identifier, residue_sequence_number,
            insertion_code, x, y, z, occupancy, temperature_factor)) //element_symbol))

    }

    fn radius(&self) -> f64 {

        f64::sqrt(self.x() * self.x() + self.y() + self.y() + self.z() * self.z())
    }


}