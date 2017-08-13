use std::collections::HashMap;

pub fn nucleotide_counts<'a>(s: &'a str) -> Result<HashMap<char, usize>, &'a str> {
    let mut m: HashMap<char, usize> = HashMap::new();
    m.insert('A', 0);
    m.insert('C', 0);
    m.insert('G', 0);
    m.insert('T', 0);

    for c in s.chars() {
        match c {
            'A' | 'C' | 'G' | 'T' => {
                let new_count = (*m.get(&c).unwrap() as u64 + 1) as usize;
                m.insert(c, new_count)
            }
            _ => return Err("invalid nucleotide"),
        };
    };

    Ok(m)
}

pub fn count<'a>(c: char, s: &'a str) -> Result<usize, &'a str> {
    let m: Result<HashMap<char, usize>, &'a str> = nucleotide_counts(s);

    match m {
        Ok(m) => {
            match c {
                'A' | 'C' | 'G' | 'T' => return Ok(*m.get(&c).unwrap()),
                _ => return Err("invalid nucleotide"),
            }
        },
        Err(msg) => return Err(msg)
    }
}