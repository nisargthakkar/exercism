#[derive(Debug)]
pub struct RibonucleicAcid {
    strand: String
}

impl PartialEq for RibonucleicAcid {
    fn eq(&self, other: &RibonucleicAcid) -> bool {
        self.strand.eq(&other.strand)
    }
}

impl RibonucleicAcid {
    pub fn new(s: &str) -> RibonucleicAcid {
        RibonucleicAcid {
            strand: s.to_string()
        }
    }
}

#[derive(Debug)]
pub struct DeoxyribonucleicAcid {
    strand: String
}

impl PartialEq for DeoxyribonucleicAcid {
    fn eq(&self, other: &DeoxyribonucleicAcid) -> bool {
        self.strand.eq(&other.strand)
    }
}

impl DeoxyribonucleicAcid {
    pub fn new(s: &str) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid {
            strand: s.to_string()
        }
    }

    pub fn to_rna(&self) -> Result<RibonucleicAcid, &'static str> {
        let mut strand = String::new();
        
        let any_invalid = self.strand.chars().any(|c: char| match c {
            'G' => {strand.push('C'); false},
            'C' => {strand.push('G'); false},
            'T' => {strand.push('A'); false},
            'A' => {strand.push('U'); false},
            _  => true,
        });

        if any_invalid {
            return Result::Err("invalid characters");
        }

        Result::Ok(RibonucleicAcid::new(&strand))
    }
}