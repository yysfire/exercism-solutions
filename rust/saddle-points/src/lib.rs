pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let row_len = input.len();
    // the column index of maximum element of one row (maybe more than one)
    let mut row_max_y = Vec::new();

    for (x, row) in input.iter().enumerate() {
        row_max_y.clear();
        // the maximum value of one row
        let mut row_max: u64 = 0;

        // find all maximum elements' column indexes in current row
        for (y, &point) in row.iter().enumerate() {
            if point > row_max {
                row_max_y.clear();
                row_max_y.push(y);
                row_max = point;
            } else if point == row_max {
                row_max_y.push(y);
            }
        }

        // check if it is the smallest element in its column
        for j in &row_max_y {
            let mut flag = true;
            for i in 0..row_len {
                if let Some(p) = input[i].get(*j) {
                    // it is not the smallest
                    if *p < row_max {
                        flag = false;
                    }
                }
            }
            if flag {
                result.push((x, *j));
            }
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_square_matrix_wide() {
        let input = vec![vec![10, 7, 10, 10, 10], vec![10, 7, 4, 17, 19]];
        assert_eq!(vec![(0, 0), (0, 3), (0, 4)], find_saddle_points(&input));
    }

}
