pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let row_count = input.len();

    let mut saddle_points = vec![];

    for (row_index, row) in input.iter().enumerate() {
        if let Some(largest_in_row) = row.iter().max() {
            for (column_index, value) in row.iter().enumerate() {
                if let Some(smallest_in_column) = (0..row_count).map(|x| input[x][column_index]).min() {
                    if value == largest_in_row && value == &smallest_in_column {
                        saddle_points.push((row_index, column_index));
                    }
                }
            }
        }
    }

    return saddle_points;
}
