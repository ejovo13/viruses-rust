// This module is used to organize atoms and molecules

mod acid {

    use strum_macros::EnumIter;

    const _NAME_ARRAY: [&str; 26] = ["Alanine", "Arginine", "Asparagine", "Aspartic acid", "Cysteine", "Glutamine", "Glutamic acid", "Glycine", "Histidine", "Isoleucine", "Leucine",
    "Lysine", "Methionine", "Phenylalanine", "Proline", "Pyrrolysine", "Serine", "Selenocysteine", "Threonine", "Tryptophan", "Tyrosine", "Valine",
    "Aspartic acid or Asparagine", "Glutamic acid or Glutamine", "Any amino acid", "Leucine or Isoleucine"];
    const _CHAR_ARRAY: [char; 26] = ['A', 'R', 'N', 'D', 'C', 'Q', 'E', 'G', 'H', 'I', 'L', 'K', 'M', 'F', 'P', 'O', 'S', 'U', 'T', 'W', 'Y', 'V', 'B', 'Z', 'X', 'J'];
    const _ABBR_ARRAY: [&str; 26] = ["Ala", "Arg", "Asn", "Asp", "Cys", "Gln", "Glu", "Gly", "His", "Ile", "Leu", "Lys", "Met", "Phe", "Pro", "Pyl", "Ser", "Sec", "Thr", "Trp", "Tyr", "Val", "Asx", "Glx", "Xaa", "Xle"];

    struct AminoAcidInfo {
        symbol: char,
        full_name: String,
        abbr: String,
    }

    impl AminoAcidInfo {
        fn new(symbol: char, full_name: &str, abbr: &str) -> Self {
            AminoAcidInfo {
                symbol,
                full_name: full_name.to_string(),
                abbr: abbr.to_string(),
            }
        }
    }

    type Info = AminoAcidInfo;

    #[derive(EnumIter, Debug)]
    enum AminoAcid {

        Ala,
        Arg,
        Asn,
        Asp,
        Cys,
        Gln,
        Glu,
        Gly,
        His,
        Ile,
        Leu,
        Lys,
        Met,
        Phe,
        Pro,
        Pyl,
        Ser,
        Sec,
        Thr,
        Trp,
        Tyr,
        Val,
        Asx,
        Glx,
        Xaa,
        Xle,
    }

    impl AminoAcid {

        // Get the Info structure depending on which amino acid
        fn info(&self) -> Info {

            match self {

                AminoAcid::Ala => Info::new('A', "Alanine", "Ala"),
                AminoAcid::Arg => Info::new('R', "Arginine", "Arg"),
                AminoAcid::Asn => Info::new('N', "Asparagine", "Asn"),
                AminoAcid::Asp => Info::new('D', "Aspartic acid", "Asp"),
                AminoAcid::Cys => Info::new('C', "Cysteine", "Cys"),
                AminoAcid::Gln => Info::new('Q', "Glutamine", "Gln"),
                AminoAcid::Glu => Info::new('E', "Glutamic acid", "Glu"),
                AminoAcid::Gly => Info::new('G', "Glycine", "Gly"),
                AminoAcid::His => Info::new('H', "Histidine", "His"),
                AminoAcid::Ile => Info::new('I', "Isoleucine", "Ile"),
                AminoAcid::Leu => Info::new('L', "Leucine", "Leu"),
                AminoAcid::Lys => Info::new('K', "Lysine", "Lys"),
                AminoAcid::Met => Info::new('M', "Methionine", "Met"),
                AminoAcid::Phe => Info::new('F', "Phenylalanine", "Phe"),
                AminoAcid::Pro => Info::new('P', "Proline", "Pro"),
                AminoAcid::Pyl => Info::new('O', "Pyrrolysine", "Pyl"),
                AminoAcid::Ser => Info::new('S', "Serine", "Ser"),
                AminoAcid::Sec => Info::new('U', "Selenocysteine", "Sec"),
                AminoAcid::Thr => Info::new('T', "Threonine", "Thr"),
                AminoAcid::Trp => Info::new('W', "Tryptophan", "Trp"),
                AminoAcid::Tyr => Info::new('Y', "Tyrosine", "Tyr"),
                AminoAcid::Val => Info::new('V', "Valine", "Val"),
                AminoAcid::Asx => Info::new('B', "Aspartic acid or Asparagine", "Asx"),
                AminoAcid::Glx => Info::new('Z', "Glutamic acid or Glutamine", "Glx"),
                AminoAcid::Xaa => Info::new('X', "Any amino acid", "Xaa"),
                AminoAcid::Xle => Info::new('J', "Leucine or Isoleucine", "Xle"),
            }
        }

        fn print_info(&self) {

            println!("{}", self);

        }

    }

    impl std::fmt::Display for AminoAcid {

        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

            let info = self.info();

            writeln!(f, "Amino Acid:   {}", info.full_name);
            writeln!(f, "Abbreviation: {}", info.abbr);
            writeln!(f, "Symbol:       {}", info.symbol)

        }
    }


    #[cfg(test)]
    mod tests {

        use strum::IntoEnumIterator;

        #[test]
        fn get_acid_and_print_info() {
            for acid in super::AminoAcid::iter() {
                println!("{}", acid);
            }
        }


    }



}

mod atom {

    enum Atom {

        H,                                                                  He,
        Li, Be,                                         B,  C,  N,  O,  F,  Ne,
        Na, Mg,                                         Al, Si, P,  S,  Cl, Ar,
        K,  Ca, Sc, Ti, V,  Cr, Mn, Fe, Co, Ni, Cu, Zn, Ga, Ge, As, Se, Br, Kr,
        Rb, Sr, Y,  Zr, Nb, Mo, Tc, Ru, Rh, Pd, Ag, Cd, In, Sn, Sb, Te, I,  Xe,
        Cs, Ba,     Hf, Ta, W,  Re, Os, Ir, Pt, Au, Hg, Tl, Pb, Bi, Po, At, Rn,
        Fr, Ra,     Rf, Db, Sg, Bh, Hs, Mt, Ds, Rg, Cn, Nh, Fl, Mc, Lv, Ts, Og,

                    La, Ce, Pr, Nd, Pm, Sm, Eu, Gd, Tb, Dy, Ho, Er, Tm, Yb, Lu,
                    Ac, Th, Pa, U,  Np, Pu, Am, Cm, Bk, Cf, Es, Fm, Md, No, Lr,
    }
}