pub fn hamming_distance(dna_1: &'static str, dna_2: &'static str) -> Result<u32, &'static str> {
    if dna_1.len() != dna_2.len() {
        return Result::Err("Strand lengths not equal");
    }

    let mut hamming_distance = 0;

    for (dna_1_nuc, dna_2_nuc) in dna_1.chars().zip(dna_2.chars()) {
        if dna_1_nuc != dna_2_nuc {
            hamming_distance += 1;
        }
    }



    return Result::Ok(hamming_distance);
}