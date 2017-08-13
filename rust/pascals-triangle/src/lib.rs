pub struct PascalsTriangle {
    row_count: u32
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self {
            row_count: row_count
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut pt: Vec<Vec<u32>> = Vec::new();
        let mut prev_row: Vec<u32> = Vec::new();

        for _ in 0..self.row_count {
            let next_row = self.next_row(prev_row);
            pt.push(next_row.clone());
            prev_row = next_row.clone();
        }

        pt
    }

    fn next_row(&self, prev_row: Vec<u32>) -> Vec<u32> {
        let mut this_row: Vec<u32> = Vec::new();
        let mut prev_row = prev_row;

        if prev_row.len() == 0 {
            return vec![1];
        }

        prev_row.push(0);
        prev_row.reverse();
        prev_row.push(0);

        for i in 0..prev_row.len()-1 {
            this_row.push(prev_row[i] + prev_row[i+1]);
        }

        this_row
    }
}
