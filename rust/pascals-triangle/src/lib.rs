pub struct PascalsTriangle {
    row_count: u32
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut result = Vec::new();
        for row in 0..self.row_count {
            let mut row_data = vec![1];
            let mut last = 1;
            for column in 0..row {
                last =  last * (row - column) / (column + 1);
                row_data.push(last)
            }
            result.push(row_data)
        }
        result
    }
}
