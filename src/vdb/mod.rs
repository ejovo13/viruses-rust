pub mod downloader;
pub mod reader;

use std::str::FromStr;
use crate::math;

type vec3<T> = [T; 3];

pub struct VDB {

    pub full_path: String,
    pub num_atoms: usize,
    // vdb_read: bool, // Determine whether the vdb has actually been read
    pub atoms: Vec<Atom>,
    pub au: bool, // If true, only load the coordinates of  the AU
    pub pdbid: String,

}

// This structure represents all of the information that can be extracted from a vdb webpage
pub struct VDBInfo {

    pub name: String,
    pub family: String,
    pub genus: String,
    pub genome: String,
    pub host: String,
    pub t_number: usize,
    pub resolution: f64,

}

pub struct Atom {

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

    fn coords(&self) -> vec3::<f64> {

        [self.x, self.y, self.z]
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

    pub fn radius(&self) -> f64 {

        f64::sqrt(self.x() * self.x() + self.y() + self.y() + self.z() * self.z())
    }
}