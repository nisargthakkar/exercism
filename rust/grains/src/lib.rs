pub fn square(s: u32) -> u64 {
    if s > 64 || s <= 0 {
        panic!("Square must be between 1 and 64");
    }

    (2 as f64).powi((s - 1) as i32) as u64
}

pub fn total() -> u64 {
    square(64) + (square(64) - 1)
}
