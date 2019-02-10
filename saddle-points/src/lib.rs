pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut res = vec![];
    for (i, row) in input.iter().enumerate() {
        let mut is_saddle = true;
        // test in the row
        for (j, item) in row.iter().enumerate() {
            if row[i] < *item {
                is_saddle = false;
            }
            // test in the col
            for (k, l) in input.iter().enumerate() {
                if row[i] > l[k] {
                    is_saddle = false;
                }
            }
            // push if ok
            if is_saddle {
                res.push((i, j));
            }
        }
    }
    res
}
