use crate::vdb::{VDB, VDBInfo, Atom};
use std::fmt::{Display, Formatter};

// Create a virus structure that stores information like where the pdb is stored on the current system
// Also tries to gather basic information about the virus
pub struct Virus {

    name: String,
    pdbid: String,
    au: bool,
    atoms: Vec<Atom>,
    genus: String,
    genome: String,
    host: String,
    t_number: usize,
    n_atoms: usize,
    resolution: f64,
    // TODO Add atoms per protein

}


impl Virus {

    pub fn from(vdb: VDB, info: VDBInfo) -> Self {

        Virus {

            name: info.name,
            pdbid: vdb.pdbid,
            au: vdb.au,
            atoms: vdb.atoms,
            genus: info.genus,
            genome: info.genome,
            host: info.host,
            t_number: info.t_number,
            resolution: info.resolution,
            n_atoms: vdb.num_atoms,
        }
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

impl Display for Virus {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        writeln!(f, "{}", self.name);
        writeln!(f, "Pdbid:             {}", self.pdbid);
        writeln!(f, "T-number:          {}", self.t_number);
        writeln!(f, "N_atoms:           {}", self.n_atoms);
        writeln!(f, "au:                {}", self.au);
        writeln!(f, "genus:             {}", self.genus);
        writeln!(f, "genome:            {}", self.genome);
        writeln!(f, "host:              {}", self.host);
        writeln!(f, "resolution:        {}", self.resolution);
        writeln!(f, "max_radius:        {}", self.max_radius());
        Ok(())
    }
}