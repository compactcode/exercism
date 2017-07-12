pub struct PascalsTriangle {
    row_count: u32
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (0..self.row_count).map(|n| PascalsTriangle::row(n)).collect()
    }

    fn row(n: u32) -> Vec<u32> {
        (0..n).fold(vec![1], |mut data, column| {
            if let Some(&last) = data.last() {
                data.push(last * (n - column) / (column + 1));
            }
            data
        })
    }
}
